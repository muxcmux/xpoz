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
  let itemInSpotlight: GalleryItem | null = null;
  const seeded = {
    first: false,
    second: false,
    third: false,
  }
  const items: {[key: string]: GalleryItem | null} = {
    first: null,
    second: null,
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

    if (fetched.length) {
      let cp = Math.floor(index / perPage);
      let cpi = index % perPage;
      let cursor = loadedPages.indexOf(cp) * perPage + cpi;

      if (!seeded.second) {
        items.second = gallery.items[cursor];
        itemInSpotlight = items.second;
        seeded.second = true;

        if (items.second.prev) {
          items.first = items.second.prev;
          seeded.first = true;
        }

        if (items.second.next) {
          items.third = items.second.next;
          seeded.third = true;
        }
      }

      // seeding the first item didn't work with the first page
      if (!seeded.first && cursor > 0 && gallery.size() > perPage) {
        items.first = gallery.items[cursor - 1];
        seeded.first = true;
      }

      // seeding the third item did't with the first page
      if (!seeded.third && index < album.photosCount + album.videosCount - 1 && gallery.size() > perPage) {
        items.third = gallery.items[cursor + 1];
        seeded.third = true;
      }
    }
  });

  onDestroy(unsubscribe);

  function next() {
    swipes -= 1;
    moveX = swipes * (viewportWidth + spacing);
    if (swipes % 3 == -1 || swipes % 3 == 2) {
      firstItemJumps += 1;
      items.first = items.third?.next || null;
      itemInSpotlight = items.third;
    }
    if (swipes % 3 == -2 || swipes % 3 == 1) {
      secondItemJumps += 1;
      items.second = items.first?.next || null;
      itemInSpotlight = items.first;
    }
    if (swipes % 3 == 0) {
      thirdItemJumps += 1;
      items.third = items.second?.next || null;
      itemInSpotlight = items.second;
    }
    replace(`${$location}?${index + 1}`);
  }

  function prev() {
    swipes += 1;
    moveX = swipes * (viewportWidth + spacing);
    if (swipes % 3 == 1 || swipes % 3 == -2) {
      thirdItemJumps -= 1;
      items.third = items.first?.prev || null;
      itemInSpotlight = items.first;
    }
    if (swipes % 3 == 2 || swipes % 3 == - 1) {
      secondItemJumps -= 1;
      items.second = items.third?.prev || null;
      itemInSpotlight = items.third;
    }
    if (swipes % 3 == 0) {
      firstItemJumps -= 1;
      items.first = items.second?.prev || null;
      itemInSpotlight = items.second;
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
  let firstWidth   = 0;
  let firstHeight  = 0;
  let firstLeft    = 0;
  let firstTop     = 0;
  let secondWidth  = 0;
  let secondHeight = 0;
  let secondLeft   = 0;
  let secondTop    = 0;
  let thirdWidth   = 0;
  let thirdHeight  = 0;
  let thirdLeft    = 0;
  let thirdTop     = 0;

  $: firstAssetWidth   = items.first?.asset.width || 0;
  $: firstAssetHeight  = items.first?.asset.height || 0;
  $: firstAssetRatio   = firstAssetWidth / firstAssetHeight;
  $: secondAssetWidth  = items.second?.asset.width || 0;
  $: secondAssetHeight = items.second?.asset.height || 0;
  $: secondAssetRatio  = secondAssetWidth / secondAssetHeight;
  $: thirdAssetWidth   = items.third?.asset.width || 0;
  $: thirdAssetHeight  = items.third?.asset.height || 0;
  $: thirdAssetRatio   = thirdAssetWidth / thirdAssetHeight;

  $: firstAssetDesiredHeight = viewportHeight;
  $: firstAssetDesiredWidth  = firstAssetDesiredHeight * firstAssetRatio;
  $: if (firstAssetDesiredWidth > viewportWidth) {
    firstWidth = viewportWidth;
    firstHeight = viewportWidth / firstAssetRatio;
    firstTop = viewportHeight / 2 - firstHeight / 2;
    firstLeft = 0;
  } else {
    firstHeight = firstAssetDesiredHeight;
    firstWidth = firstAssetDesiredWidth;
    firstTop = 0;
    firstLeft = viewportWidth / 2 - firstWidth / 2;
  }

  $: secondAssetDesiredHeight = viewportHeight;
  $: secondAssetDesiredWidth  = secondAssetDesiredHeight * secondAssetRatio;
  $: if (secondAssetDesiredWidth > viewportWidth) {
    secondWidth = viewportWidth;
    secondHeight = viewportWidth / secondAssetRatio;
    secondTop = viewportHeight / 2 - secondHeight / 2;
    secondLeft = 0;
  } else {
    secondHeight = secondAssetDesiredHeight;
    secondWidth = secondAssetDesiredWidth;
    secondTop = 0;
    secondLeft = viewportWidth / 2 - secondWidth / 2;
  }

  $: thirdAssetDesiredHeight = viewportHeight;
  $: thirdAssetDesiredWidth  = thirdAssetDesiredHeight * thirdAssetRatio;
  $: if (thirdAssetDesiredWidth > viewportWidth) {
    thirdWidth = viewportWidth;
    thirdHeight = viewportWidth / thirdAssetRatio;
    thirdTop = viewportHeight / 2 - thirdHeight / 2;
    thirdLeft = 0;
  } else {
    thirdHeight = thirdAssetDesiredHeight;
    thirdWidth = thirdAssetDesiredWidth;
    thirdTop = 0;
    thirdLeft = viewportWidth / 2 - thirdWidth / 2;
  }

  let panning: "vertical" | "horizontal" | null;
  let backdropOpacity = 1;
  let moveX = 0;
  let currY = 0;
  let xOffset = 0;
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
      // xOffset = e.detail.deltaX * damping;
    } else {
      moveX = swipes * (viewportWidth + spacing) + e.detail.deltaX * damping;
    }
  }

  $: firstX = (-viewportWidth - spacing) + firstItemJumps * (3 * (viewportWidth + spacing)) + xOffset;
  $: thirdX = (viewportWidth + spacing) + thirdItemJumps * (3 * (viewportWidth + spacing)) + xOffset;
  $: secondX = (secondItemJumps * (3 * (viewportWidth + spacing))) + xOffset;

</script>

<svelte:window bind:innerWidth={viewportWidth} bind:innerHeight={viewportHeight} />

<div class="spotlight" transition:fade={{ duration: 150 }}>
  <div class="backdrop {panning == "vertical" ? 'no-transition' : ''}" style="opacity: {backdropOpacity}" on:click={() => replace($location)}></div>

  <div class="swiper" use:touch on:panmove={move} on:panend={stopMoving}>
    <div class="assets {panning ? 'no-transition' : ''}"
         style="transform: translate3d({moveX}px, 0px, 0px)">

      {#if items.first}
        <div class="asset no-transition"
             style="width: {firstWidth}px;
                    height: {firstHeight}px;
                    left: {firstLeft}px;
                    top: {firstTop}px;
                    transform: translate3d({firstX}px, {currY}px, 0px) scale({scaling})">

          <ImageLoader uuid={items.first.uuid}
                      variant="resized"
                      alt={items.first.uuid} />
        </div>
      {/if}

      {#if items.second}
        <div class="asset no-transition"
            style="width: {secondWidth}px;
                    height: {secondHeight}px;
                    left: {secondLeft}px;
                    top: {secondTop}px;
                    transform: translate3d({secondX}px, {currY}px, 0px) scale({scaling})">

          <ImageLoader uuid={items.second.uuid}
                      variant="resized"
                      alt={items.second.uuid}
                      on:click={() => replace($location)} />
        </div>
      {/if}

      {#if items.third}
        <div class="asset no-transition"
            style="width: {thirdWidth}px;
                    height: {thirdHeight}px;
                    left: {thirdLeft}px;
                    top: {thirdTop}px;
                    transform: translate3d({thirdX}px, {currY}px, 0px) scale({scaling})">

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
        <li class="{itemInSpotlight?.uuid == item.uuid ? 'selected' : ''}" transition:scale>
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


