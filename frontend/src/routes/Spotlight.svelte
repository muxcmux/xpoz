<style lang="scss">
  .backdrop {
    background: black;
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: 5;

    img {
      object-fit: contain;
      width: 100%;
      height: 100%;
    }

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

        img {
          object-fit: cover;
          width: 100%;
          height: 100%;
        }
      }
    }
  }

  button {
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
  import loadable from "../use/loadable";
  import { fade, scale } from "svelte/transition";
  import { location, replace } from "svelte-spa-router";
  import type { Asset, Album } from "src/codegen/types";
  import { getAlbum } from "../gql/albums";
  import { operationStore, query } from "@urql/svelte";
  import { onDestroy } from "svelte";

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
  $: {
    console.log("currentPage, cursor, currentPageIndex, loadedPages", currentPage, cursor, currentPageIndex, loadedPages);
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

<div class="backdrop" transition:fade={{ duration: 100 }} on:click={() => replace($location)}>
  {#if currAsset}
    <img use:loadable={{ uuid: currAsset.uuid, variant: "resized" }}
         alt={currAsset.uuid}
         on:click={() => replace($location)} />

    <ul>
      {#each items as item}
        <li class="{currAsset.uuid == item.uuid ? 'selected' : ''}" transition:scale>
          <img use:loadable={{uuid: item.uuid, variant: "thumb"}} alt={item.uuid}>
        </li>
      {/each}
    </ul>
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

  {#if $req.error}
    <p class="error">
      😵 {$req.error?.message}
    </p>
  {/if}
</div>
