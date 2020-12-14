<style lang="scss">
  .no-transition { transition: none !important }

  .spotlight,
  .backdrop,
  .swiper,
  .assets {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 5;
  }

  .backdrop {
    background: black;
    transition: .6s opacity cubic-bezier(0.215, 0.610, 0.355, 1);
  }

  .swiper {
    overflow: hidden;
    perspective: 70em;
    z-index: 6;
  }

  .assets {
    position: absolute;
    transition: .6s transform cubic-bezier(0.215, 0.610, 0.355, 1), .6s opacity cubic-bezier(0.215, 0.610, 0.355, 1);
  }

  .asset {
    position: fixed;

    &.assetAnimatedTransition {
      transition: .3s transform cubic-bezier(0.25, 0.46, 0.45, 0.94);
    }

    :global(.image-loader) {
      &.loading {
        @keyframes loader {
          0% {
            transform: scale(0);
            opacity: 0;
          }
          50% {
            opacity: .8;
          }
          100% {
            transform: scale(1);
            opacity: 0;
          }
        }

        &::before, &::after {
          z-index: 10;
          content: '';
          width: 1.5em;
          height: 1.5em;
          border: .2em solid white;
          border-radius: 50%;
          position: absolute;
          top: 1em;
          right: 1em;
          animation: loader .6s cubic-bezier(0.25, 0.46, 0.45, 0.94) infinite;
          opacity: 0;
        }

        &::before {
          animation-delay: .6s;
        }

        &::after {
          animation-delay: .2s;
        }
      }
    }
  }

  .film-strip {
    position: fixed;
    z-index: 7;
    left: 0;
    right: 0;
    bottom: 0;

    ul {
      background: rgba(0,0,0,.9);
      list-style: none;
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      margin: 0;
      padding: 10px;
      display: flex;
      justify-content: center;

      li {
        width: 40px;
        height: 40px;
        overflow: hidden;
        margin: 0 1px;
        border: 2px solid transparent;

        &.selected {
          border-color: red;
        }

        :global(img) {
          object-fit: cover;
        }
      }
    }
  }

  button {
    z-index: 8;
    appearance: none;
    border: none;
    background: rgba(255, 255, 255, .15);
    position: absolute;
    top: 50%;
    transform: translate(0, -50%);
    width: 3em;
    height: 3em;
    border-radius: .5em;
    padding: .8em;
    color: rgba(255, 255, 255, .5);
    outline: none;
    cursor: pointer;
    opacity: .5;
    transition: opacity .2s ease;

    &:hover { opacity: 1 }

    svg {
      width: 100%;
      height: 100%;
      stroke-width: 2;
    }

    &.prev {
      left: 1em;
    }

    &.next {
      right: 1em;
    }

    @media (hover: none) {
      display: none;
    }
  }

</style>

<script lang="ts">
  import { fade, scale, fly } from "svelte/transition";
  import { sineOut } from "svelte/easing";
  import { location, replace, pop } from "svelte-spa-router";
  import type { Asset, Album } from "src/codegen/types";
  import { getAlbum } from "../gql/albums";
  import { operationStore, query } from "@urql/svelte";
  import { onDestroy } from "svelte";
  import ImageLoader from "../components/ImageLoader.svelte";
  import touch from "../use/touch";
  import { Gallery, GalleryItem } from "../lib/gallery";
  import { CarouselItem } from "../lib/carousel";
  import { between, clamp, outOfBounds, roundBounds, roundPoint } from "../utils/math";
  import type { Point, Bounds } from "../utils/math";

  export let album: Album;
  export let index: number;
  export let perPage: number;

  enum InsertPosition {
    Right,
    Left
  }

  let dir = (index % perPage) <= 1 ? InsertPosition.Left : InsertPosition.Right;
  let loadPage = Math.floor(index / perPage);
  let loadedPages = [loadPage];
  let lastPage = Math.floor((album.photosCount + album.videosCount) / perPage);
  let gallery = new Gallery<Asset>();
  let current: string = "second";
  const seeded = {
    first: false,
    second: false,
    third: false,
  }
  const carousel: {[key: string]: CarouselItem} = {
    first: new CarouselItem(-1),
    second: new CarouselItem(0),
    third: new CarouselItem(1),
  }

  const req = operationStore(getAlbum, {
    uuid: album.uuid,
    offset: loadPage * perPage,
    limit: perPage,
  });

  query(req);

  const unsubscribe = req.subscribe(value => {
    let fetched = value.data?.album?.assets as Asset[] || [];
    if(dir == InsertPosition.Right) {
      gallery = gallery.append(fetched);
      if (!loadedPages.includes(loadPage)) loadedPages = [...loadedPages, loadPage];
    } else {
      gallery = gallery.prepend(fetched);
      if (!loadedPages.includes(loadPage)) loadedPages = [loadPage, ...loadedPages];
    }

    if (fetched.length) {
      let cp = Math.floor(index / perPage);
      let cpi = index % perPage;
      let cursor = loadedPages.indexOf(cp) * perPage + cpi;

      if (!seeded.second) {
        carousel.second = carousel.second.setItem(gallery.items[cursor]);
        seeded.second = true;

        if (carousel.second!.item!.prev) {
          carousel.first = carousel.first.setItem(carousel.second!.item!.prev);
          seeded.first = true;
        }

        if (carousel.second!.item!.next) {
          carousel.third = carousel.third.setItem(carousel.second!.item!.next);
          seeded.third = true;
        }
      }

      // seeding the first item didn't work with the first page
      if (!seeded.first && cursor > 0 && gallery.size() > perPage) {
        carousel.first = carousel.first.setItem(gallery.items[cursor - 1]);
        seeded.first = true;
      }

      // seeding the third item did't with the first page
      if (!seeded.third && index < album.photosCount + album.videosCount - 1 && gallery.size() > perPage) {
        carousel.third = carousel.third.setItem(gallery.items[cursor + 1]);
        seeded.third = true;
      }
    }
  });

  onDestroy(unsubscribe);

  $: currentPage = Math.floor(index / perPage);
  $: currentPageIndex = index % perPage;
  $: hasPrev   = index > 0;
  $: hasNext   = index < album.photosCount + album.videosCount - 1;

  $: if (!$req.fetching && currentPageIndex >= perPage - 3) loadNextPage()
  $: if (!$req.fetching && currentPageIndex <= 2) loadPrevPage()

  function loadPrevPage() {
    dir = InsertPosition.Left;
    if (currentPage > 0 && !loadedPages.includes(currentPage - 1)) {
      loadPage = currentPage - 1;
      $req.variables!.offset = loadPage * perPage;
    }
  }

  function loadNextPage() {
    dir = InsertPosition.Right;
    if (currentPage < lastPage && !loadedPages.includes(currentPage + 1)) {
      loadPage = currentPage + 1;
      $req.variables!.offset = loadPage * perPage;
    }
  }

  function keyboardNav(e: KeyboardEvent) {
    if (e.key == "ArrowLeft") prev();
    if (e.key == "ArrowRight") next();
    if (e.key == "ArrowUp" || e.key == "Escape" || e.key == "ArrowDown") {
      opacity = 0;
      backdropOpacity = 0;
      moveY = viewportHeight * (e.key == "ArrowDown" ? 1 : -1);
      setTimeout(pop, 300);
    }
  }

  let viewportHeight: number;
  let viewportWidth: number;

  function onResize() {
    setTimeout(() => {
      moveX = swipes * (viewportWidth + spacing);
    }, 10)
    for (const [key, c] of Object.entries(carousel)) {
      carousel[key] = carousel[key].setItem(c.item);
    }
  }

  let panning: "vertical" | "horizontal" | null;
  let opacity = 1;
  let backdropOpacity = 1;
  let moveX = 0;
  let moveY = 0;
  let swipes = 0;
  let zooming = false;
  const spacing = 20;
  $: panThresholdForChange = viewportWidth / 5;
  $: panThresholdForClose = viewportHeight / 6;

  function next() {
    swipes -= 1;
    moveX = swipes * (viewportWidth + spacing);
    if (swipes % 3 == 1 || swipes % 3 == -2) {
      carousel.second.jumps += 1;
      carousel.second.setItem(carousel.first!.item?.next || null);
      current = "first";
    }
    if (swipes % 3 == 2 || swipes % 3 == -1) {
      carousel.first.jumps += 1;
      carousel.first.setItem(carousel.third!.item?.next || null);
      current = "third";
    }
    if (swipes % 3 == 0) {
      carousel.third.jumps += 1;
      carousel.third.setItem(carousel.second!.item?.next || null);
      current = "second";
    }
    replace(`${$location}?${index + 1}`);
  }

  function prev() {
    swipes += 1;
    moveX = swipes * (viewportWidth + spacing);
    if (swipes % 3 == 1 || swipes % 3 == -2) {
      carousel.third.jumps -= 1;
      carousel.third.setItem(carousel.first!.item?.prev || null);
      current = "first";
    }
    if (swipes % 3 == 2 || swipes % 3 == - 1) {
      carousel.second.jumps -= 1;
      carousel.second.setItem(carousel.third!.item?.prev || null);
      current = "third";
    }
    if (swipes % 3 == 0) {
      carousel.first.jumps -= 1;
      carousel.first.setItem(carousel.second!.item?.prev || null);
      current = "second";
    }
    replace(`${$location}?${index - 1}`);
  }

  function stopMoving(e: CustomEvent) {
    if (zooming) return stopZoomedMoving(e);

    opacity = 1;
    backdropOpacity = 1;
    moveY = 0;

    if (panning == "horizontal") {
      if (hasPrev && e.detail.deltaX > panThresholdForChange) {
        prev();
      } else if (hasNext && e.detail.deltaX < -panThresholdForChange) {
        next();
      } else {
        // reset
        moveX = swipes * (viewportWidth + spacing);
      }
    } else {
      if (Math.abs(e.detail.deltaY) > panThresholdForClose) {
        opacity = 0;
        backdropOpacity = 0;
        moveY = viewportHeight * Math.sign(e.detail.deltaY);
        setTimeout(pop, 300);
      }
    }
    panning = null;
  }

  function startMoving(e: CustomEvent) {
    if (zooming) return startZoomedMoving(e);
  }

  function move(e: CustomEvent) {
    if (zooming) return zoomedMove(e);

    if (!panning) {
      panning = ["pandown", "panup"].includes(e.detail.additionalEvent) ? panning = "vertical" : "horizontal";
    }

    if (panning == "vertical") {
      moveY = e.detail.deltaY;
      backdropOpacity = 1 - (Math.abs(e.detail.deltaY) * 1.5) / viewportHeight;
    } else {
      moveX = swipes * (viewportWidth + spacing) + e.detail.deltaX * 0.85;
    }
  }

  function zoom(e: CustomEvent) {
    zooming = !zooming;
    assetAnimatedTransition = true;
    panDelta = { x: 0, y: 0 };

    if (zooming) {
      panOrigin = {
        x: carousel[current].x,
        y: carousel[current].y,
      }

      // Work out correct zoom with device pixel ratio and big image dimensions
      // This will be useful when loading the render or original or zoom
      // For now just zoom enough to fill in the screen
      let zoom = [2];
      zoom.push(viewportWidth / carousel[current].width);
      zoom.push(viewportHeight / carousel[current].height);
      carousel[current].scale = Math.max(...zoom);

      const diffX = (carousel[current].width * carousel[current].scale - Math.min(viewportWidth, carousel[current].width * carousel[current].scale)) / 2;
      const diffY = (carousel[current].height * carousel[current].scale - Math.min(viewportHeight, carousel[current].height * carousel[current].scale)) / 2;

      panBounds = {
        min: { x: -diffX, y: -diffY },
        max: { x: diffX, y: diffY }
      }
    } else {
      carousel[current].scale = 1;
      carousel[current].x = panOrigin.x;
      carousel[current].y = panOrigin.y;
      setTimeout(() => assetAnimatedTransition = false, 300);
    }
  }

  function startZoomedMoving(e: CustomEvent) {
    assetAnimatedTransition = false;
    cancelAnimationFrame(rAF);
    currentMoveWithinBoundsDelta = { x: 0, y: 0 };
    currentMoveOutOfBoundsDelta = { x: 0, y: 0 };
  }

  function zoomedMove(e: CustomEvent) {
    const bounds = carousel[current].getBoundsForOrigin(roundPoint(panOrigin));

    const oob = outOfBounds(roundPoint({
      x: panOrigin.x + panDelta.x + e.detail.deltaX,
      y: panOrigin.y + panDelta.y + e.detail.deltaY
    }), bounds);

    currentMoveOutOfBoundsDelta = {
      x: oob && oob.x != 0 ? (e.detail.deltaX - currentMoveWithinBoundsDelta.x) * 0.5 : currentMoveOutOfBoundsDelta.x,
      y: oob && oob.y != 0 ? (e.detail.deltaY - currentMoveWithinBoundsDelta.y) * 0.5 : currentMoveOutOfBoundsDelta.y,
    }

    currentMoveWithinBoundsDelta = {
      x: oob && oob.x != 0 ? currentMoveWithinBoundsDelta.x : e.detail.deltaX,
      y: oob && oob.y != 0 ? currentMoveWithinBoundsDelta.y : e.detail.deltaY,
    }

    const x = panOrigin.x + panDelta.x + currentMoveDelta.x;
    const y = panOrigin.y + panDelta.y + currentMoveDelta.y;

    carousel[current].x = x;
    carousel[current].y = y;
  }

  function stopZoomedMoving(e: CustomEvent) {
    const decelerationAmmount = {
      x: Math.min(Math.abs(e.detail.velocityX), 1.3) * 12 * Math.sign(e.detail.velocityX),
      y: Math.min(Math.abs(e.detail.velocityY), 1.3) * 12 * Math.sign(e.detail.velocityY),
    }

    console.log(e.detail);

    const deltaX = panDelta.x + currentMoveDelta.x;
    const deltaY = panDelta.y + currentMoveDelta.y;

    panDelta.x = deltaX;
    panDelta.y = deltaY;

    requestAnimationFrame(() => {
      decelerate({ x: 0.92, y: 0.92 }, decelerationAmmount);
    });
  }

  function decelerate(friction: Point, { x, y }: Point) {
    const oob = outOfBounds(roundPoint(panDelta), roundBounds(panBounds));

    const delta = { x: x * friction.x, y: y * friction.y };

    panDelta.x += delta.x;
    panDelta.y += delta.y;

    requestAnimationFrame(() => {
      carousel[current].x = panOrigin.x + panDelta.x;
      carousel[current].y = panOrigin.y + panDelta.y;
    })

    if (oob && oob.x != 0) {
      friction.x = 0.5;
      if (between(delta.x, -1, 1)) {
        rAF = requestAnimationFrame(timestamp => {
          moveBackWithinBounds("x", panDelta.x, (oob as Point).x, timestamp, timestamp);
        })
      }
    }

    if (oob && oob.y != 0) {
      friction.y = 0.5
      if (between(delta.y, -1, 1)) {
        rAF = requestAnimationFrame(timestamp => {
          moveBackWithinBounds("y", panDelta.y, (oob as Point).y, timestamp, timestamp);
        })
      }
    }

    if (between(delta.x, -1, 1) && between(delta.y, -1, 1)) return;

    rAF = requestAnimationFrame(() => {
      decelerate(friction, delta);
    });
  }

  function moveBackWithinBounds(coord: "x" | "y", initialValue: number, value: number, start: DOMHighResTimeStamp, time: DOMHighResTimeStamp) {
    const duration = 300;
    const elapsed = time - start;
    const change = sineOut(elapsed / duration) * value;

    panDelta[coord] = initialValue + change;
    carousel[current][coord] = panOrigin[coord] + panDelta[coord];

    if (elapsed < duration) {
      rAF = requestAnimationFrame(timestamp => {
        moveBackWithinBounds(coord, initialValue, value, start, timestamp);
      });
    }
  }

  let rAF: number;

  let currentMoveWithinBoundsDelta: Point = { x: 0, y: 0 };
  let currentMoveOutOfBoundsDelta: Point = { x: 0, y: 0 };
  $: currentMoveDelta = {
    x: currentMoveWithinBoundsDelta.x + currentMoveOutOfBoundsDelta.x,
    y: currentMoveWithinBoundsDelta.y + currentMoveOutOfBoundsDelta.y,
  }
  let panOrigin: Point = { x: 0, y: 0 };
  let panDelta: Point = { x: 0, y: 0 };
  let panBounds: Bounds = { min: { x: 0, y: 0 }, max: { x: 0, y: 0 } };
  let assetAnimatedTransition = false;

</script>

<svelte:window on:resize={onResize} on:keyup={keyboardNav} bind:innerWidth={viewportWidth} bind:innerHeight={viewportHeight} />

<div class="spotlight" in:fade={{ duration: 150 }}>
  <div class="backdrop {panning == "vertical" ? 'no-transition' : ''}"
       style="opacity: {backdropOpacity}"
       on:click={() => replace($location)}></div>

  <div class="swiper" use:touch on:panmove={move} on:panstart={startMoving} on:panend={stopMoving} on:doubletap={zoom}>
    <div class="assets {panning ? 'no-transition' : ''}"
         style="transform: translate3d({moveX}px, {moveY}px, 0px); opacity: {opacity}">
      {#each Object.entries(carousel) as [_, c]}
        {#if c.item}
          <div class="asset" class:assetAnimatedTransition
              style="width: {c.width}px;
                      height: {c.height}px;
                      left: {c.left}px;
                      top: {c.top}px;
                      transform: translate3d({c.x}px, {c.y}px, 0px) scale({c.scale})">

            <ImageLoader uuid={c.item.uuid} variant="resized" alt={c.item.uuid} />
          </div>
        {/if}
      {/each}
    </div>
  </div>

  <div class="film-strip" in:fly={{ y: 60, duration: 500}}>
    {#if $req.error}
      <p class="error">
        😵 {$req.error?.message}
      </p>
    {/if}

    <!-- <ul>
      {#each gallery.items as item}
        <li class="{itemInSpotlight?.item?.uuid == item.uuid ? 'selected' : ''}" in:scale>
          <ImageLoader uuid={item.uuid} variant="thumb" alt={item.uuid} />
        </li>
      {/each}
    </ul> -->
  </div>

  {#if hasPrev}
    <button class="prev" on:click|stopPropagation={prev} disabled={$req.fetching}>
      <svg><use xlink:href="#i-chevron-left"/></svg>
    </button>
  {/if}

  {#if hasNext}
    <button class="next" on:click|stopPropagation={next} disabled={$req.fetching}>
      <svg><use xlink:href="#i-chevron-right"/></svg>
    </button>
  {/if}
</div>
