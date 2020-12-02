<style lang="scss">
  .backdrop {
    background: black;
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: 5;
  }

  .asset {
    position: fixed;
    z-index: 6;

    :global(.image-loader) {
      &.loading {
        &::after {
          content: "";
          position: absolute;
          bottom: 100px;
          left: 20px;
          width: 20px;
          height: 20px;
          border-radius: 10px;
          background: white;
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
  }

</style>

<script lang="ts">
  import { fade, scale } from "svelte/transition";
  import { location, replace } from "svelte-spa-router";
  import type { Asset, Album } from "src/codegen/types";
  import { getAlbum } from "../gql/albums";
  import { operationStore, query } from "@urql/svelte";
  import { onDestroy } from "svelte";
  import ImageLoader from "../components/ImageLoader.svelte";

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
  let items: Asset[] = [];

  const req = operationStore(getAlbum, {
    uuid: album.uuid,
    offset: loadPage * perPage,
    limit: perPage,
  });

  query(req);

  const unsubscribe = req.subscribe(value => {
    let fetched = value.data?.album?.assets as Asset[] || [];
    let add: Asset[] = [];
    let uuids = items.map((i) => i.uuid);
    fetched.forEach((item) => {
      if(!uuids.includes(item.uuid)) add.push(item);
    });
    if(dir == InsertPosition.Right) {
      items = [...items, ...add];
      if (!loadedPages.includes(loadPage)) loadedPages = [...loadedPages, loadPage];
    } else {
      items = [...add, ...items];
      if (!loadedPages.includes(loadPage)) loadedPages = [loadPage, ...loadedPages];
    }
  });

  onDestroy(unsubscribe);

  function next() {
    if (hasNext) {
      replace(`${$location}?${index + 1}`);
    }
  }

  function prev() {
    if (hasPrev) {
      replace(`${$location}?${index - 1}`)
    }
  }

  $: currentPage = Math.floor(index / perPage);
  $: currentPageIndex = index % perPage;
  $: hasPrev   = index > 0;
  $: hasNext   = index < album.photosCount + album.videosCount - 1;
  $: cursor    = items.length? (loadedPages.indexOf(currentPage) * perPage) + currentPageIndex : 0;
  $: currAsset = items[cursor];

  let viewportHeight: number;
  let viewportWidth: number;
  let width  = 0;
  let height = 0;
  let left   = 0;
  let top    = 0;

  $: assetWidth  = currAsset?.width | 0;
  $: assetHeight = currAsset?.height | 0;
  $: assetRatio = assetWidth / assetHeight;

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

  $: if (!$req.fetching && currentPageIndex >= perPage - 2) loadNextPage()
  $: if (!$req.fetching && currentPageIndex <= 1) loadPrevPage()

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
</script>

<svelte:window bind:innerWidth={viewportWidth} bind:innerHeight={viewportHeight} />

<div class="backdrop" transition:fade={{ duration: 100 }} on:click={() => replace($location)}></div>

{#if currAsset}
  <div class="asset" style="width: {width}px; height: {height}px; left: {left}px; top: {top}px;">
    <ImageLoader uuid={currAsset.uuid}
                 variant="resized"
                 alt={currAsset.uuid}
                 on:click={() => replace($location)} />
  </div>
{/if}

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


<div class="film-strip">
  {#if $req.error}
    <p class="error">
      😵 {$req.error?.message}
    </p>
  {/if}

  <ul>
    {#each items as item}
      <li class="{currAsset.uuid == item.uuid ? 'selected' : ''}" transition:scale>
        <ImageLoader uuid={item.uuid} variant="thumb" alt={item.uuid} />
      </li>
    {/each}
  </ul>
</div>
