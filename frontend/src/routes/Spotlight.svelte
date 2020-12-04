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
    z-index: 6;
  }

  .assets {
    position: absolute;
    transition: .6s transform cubic-bezier(0.215, 0.610, 0.355, 1);
  }

  .asset {
    position: fixed;
    transition: .6s transform cubic-bezier(0.215, 0.610, 0.355, 1);

    &.no-transition {
      transition: none;
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
  import { location, replace } from "svelte-spa-router";
  import type { Asset, Album } from "src/codegen/types";
  import { getAlbum } from "../gql/albums";
  import { operationStore, query } from "@urql/svelte";
  import { onDestroy } from "svelte";
  import ImageLoader from "../components/ImageLoader.svelte";
  import touch from "../use/touch";
  import { Gallery, GalleryItem } from "../lib/gallery";

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
  let gallery = new Gallery();
  let seeded = false;
  let items: {[key: string]: GalleryItem | null} = {
    second: null,
    first: null,
    third: null,
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
    if (!seeded && fetched.length) {
      items.second = gallery.items[index % perPage];
      items.third = items.second.next;
      items.first = items.second.prev;
      seeded = true;
    }
  });

  onDestroy(unsubscribe);

  function next() {
    swipes -= 1;
    moveX = swipes * (viewportWidth + spacing);
    if (swipes % 3 == -1 || swipes % 3 == 2) {
      firstItemJumps += 1;
      items.first = items.third?.next || null;
    }
    if (swipes % 3 == -2 || swipes % 3 == 1) {
      secondItemJumps += 1;
      items.second = items.first?.next || null;
    }
    if (swipes % 3 == 0) {
      thirdItemJumps += 1;
      items.third = items.second?.next || null;
    }
    replace(`${$location}?${index + 1}`);
  }

  function prev() {
    swipes += 1;
    moveX = swipes * (viewportWidth + spacing);
    if (swipes % 3 == 1 || swipes % 3 == -2) {
      thirdItemJumps -= 1;
      items.third = items.first?.prev || null;
    }
    if (swipes % 3 == 2 || swipes % 3 == - 1) {
      secondItemJumps -= 1;
      items.second = items.third?.prev || null;
    }
    if (swipes % 3 == 0) {
      firstItemJumps -= 1;
      items.first = items.second?.prev || null;
    }
    replace(`${$location}?${index - 1}`);
  }

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

  let viewportHeight: number;
  let viewportWidth: number;
  let width      = 0;
  let height     = 0;
  let left       = 0;
  let top        = 0;
  let prevWidth  = 0;
  let prevHeight = 0;
  let prevLeft   = 0;
  let prevTop    = 0;
  let nextWidth  = 0;
  let nextHeight = 0;
  let nextLeft   = 0;
  let nextTop    = 0;

  $: assetWidth  = items.second?.asset.width || 0;
  $: assetHeight = items.second?.asset.height || 0;
  $: assetRatio = assetWidth / assetHeight;
  $: prevAssetWidth  = items.first?.asset.width || 0;
  $: prevAssetHeight = items.first?.asset.height || 0;
  $: prevAssetRatio = prevAssetWidth / prevAssetHeight;
  $: nextAssetWidth  = items.third?.asset.width || 0;
  $: nextAssetHeight = items.third?.asset.height || 0;
  $: nextAssetRatio = nextAssetWidth / nextAssetHeight;

  $: dheight = viewportHeight;
  $: dwidth  = dheight * assetRatio;
  $: if (dwidth > viewportWidth) {
    width = viewportWidth;
    height = viewportWidth / assetRatio;
    top = viewportHeight / 2 - height / 2;
    left = 0;
  } else {
    height = dheight;
    width = dwidth;
    top = 0;
    left = viewportWidth / 2 - width / 2;
  }
  $: pdheight = viewportHeight;
  $: pdwidth  = pdheight * prevAssetRatio;
  $: if (pdwidth > viewportWidth) {
    prevWidth = viewportWidth;
    prevHeight = viewportWidth / prevAssetRatio;
    prevTop = viewportHeight / 2 - prevHeight / 2;
    prevLeft = 0;
  } else {
    prevHeight = pdheight;
    prevWidth = pdwidth;
    prevTop = 0;
    prevLeft = viewportWidth / 2 - prevWidth / 2;
  }
  $: ndheight = viewportHeight;
  $: ndwidth  = ndheight * nextAssetRatio;
  $: if (ndwidth > viewportWidth) {
    nextWidth = viewportWidth;
    nextHeight = viewportWidth / nextAssetRatio;
    nextTop = viewportHeight / 2 - nextHeight / 2;
    nextLeft = 0;
  } else {
    nextHeight = ndheight;
    nextWidth = ndwidth;
    nextTop = 0;
    nextLeft = viewportWidth / 2 - nextWidth / 2;
  }

  let panning: "vertical" | "horizontal" | null;
  let backdropOpacity = 1;
  let moveX = 0;
  let currY = 0;
  let secondXOffset = 0;
  let firstItemJumps = 0;
  let thirdItemJumps = 0;
  let secondItemJumps = 0;
  let swipes = 0;
  let scaling = 1;
  const spacing = 20;
  const damping = 0.7;
  const animationLength = 600;
  $: panThresholdForChange = viewportWidth / 4;

  function stopMoving(e: CustomEvent) {
    backdropOpacity = 1;
    scaling = 1;

    currY = 0;

    if (panning == "horizontal") {
      panning = null;
      if (hasPrev && e.detail.deltaX > panThresholdForChange) {
        prev();
      } else if (hasNext && e.detail.deltaX < -panThresholdForChange) {
        next();
      } else {
        // reset
        moveX = swipes * (viewportWidth + spacing);
      }
    } else {
      panning = null;
    }
  }

  function move(e: CustomEvent) {
    if (!panning) {
      panning = ["pandown", "panup"].includes(e.detail.additionalEvent) ? panning = "vertical" : "horizontal";
    }

    if (panning == "vertical") {
      currY = e.detail.deltaY * damping;
      backdropOpacity = 1 - (Math.abs(e.detail.deltaY) * damping) / viewportHeight;
      scaling = 1 - (Math.abs(e.detail.distance) * damping) / Math.sqrt((viewportHeight ** 2) + (viewportWidth ** 2));
      secondXOffset = e.detail.deltaX * damping;
    } else {
      moveX = swipes * (viewportWidth + spacing) + e.detail.deltaX * damping;
    }
  }

  $: firstX = (-viewportWidth - spacing) + firstItemJumps * (3 * (viewportWidth + spacing));
  $: thirdX = (viewportWidth + spacing) + thirdItemJumps * (3 * (viewportWidth + spacing));
  $: secondX = (secondItemJumps * (3 * (viewportWidth + spacing))) + secondXOffset;
</script>

<svelte:window bind:innerWidth={viewportWidth} bind:innerHeight={viewportHeight} />

<div class="spotlight" transition:fade={{ duration: 150 }}>
  <div class="backdrop {panning == "vertical" ? 'no-transition' : ''}" style="opacity: {backdropOpacity}" on:click={() => replace($location)}></div>

  <div class="swiper" use:touch on:panmove={move} on:panend={stopMoving}>
    <div class="assets {panning ? 'no-transition' : ''}"
         style="transform: translate3d({moveX}px, 0px, 0px)">

      {#if items.first}
        <div class="asset no-transition"
            style="width: {prevWidth}px;
                    height: {prevHeight}px;
                    left: {prevLeft}px;
                    top: {prevTop}px;
                    transform: translate3d({firstX}px, {currY}px, 0px)">

          <ImageLoader uuid={items.first.uuid}
                      variant="resized"
                      alt={items.first.uuid} />
        </div>
      {/if}

      {#if items.second}
        <div class="asset no-transition"
            style="width: {width}px;
                    height: {height}px;
                    left: {left}px;
                    top: {top}px;
                    transform: translate3d({secondX}px, {currY}px, 0px) scale({scaling})">

          <ImageLoader uuid={items.second.uuid}
                      variant="resized"
                      alt={items.second.uuid}
                      on:click={() => replace($location)} />
        </div>
      {/if}

      {#if items.third}
        <div class="asset no-transition"
            style="width: {nextWidth}px;
                    height: {nextHeight}px;
                    left: {nextLeft}px;
                    top: {nextTop}px;
                    transform: translate3d({thirdX}px, {currY}px, 0px)">

          <ImageLoader uuid={items.third.uuid}
                      variant="resized"
                      alt={items.third.uuid} />
        </div>
      {/if}
    </div>
  </div>

  <div class="film-strip" transition:fly={{ y: 60, duration: 500}}>
    {#if $req.error}
      <p class="error">
        😵 {$req.error?.message}
      </p>
    {/if}

    <ul>
      {#each gallery.items as item}
        <li class="{items.second?.uuid == item.uuid ? 'selected' : ''}" transition:scale>
          <ImageLoader uuid={item.uuid} variant="thumb" alt={item.uuid} />
        </li>
      {/each}
    </ul>
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


