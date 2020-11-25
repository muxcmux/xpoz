<style lang="scss">
  .results {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-auto-rows: calc((100vw - 6px) / 4);
    gap: 2px;

  }

  figure {
    position: relative;
    margin: 0;
    width: 100%;
    height: 100%;
  }

  img {
    object-fit: cover;
    width: 100%;
    height: 100%;
  }

  header { margin: 1em .6em; }

  h1 {
    margin: 0;
    color: var(--color-fg);
    font-weight: 700;
    font-size: 1.1em;
    display: flex;
    align-items: center;
    justify-content: space-between;

    a {
      display: block;
      height: 1.3em;
      width: 1.2em;

      svg {
        width: 100%;
        height: 100%;
      }
    }

    span {
      margin-right: 1.1em;
      flex: 1;
      padding: 0 .5em;
      text-align: center;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }

  h2 {
    color: var(--color-fg);
    font-weight: 400;
    font-size: 1em;
    opacity: .5;
    margin-top: .5em;
    margin-bottom: .5em;
    text-align: center;

    span {
      padding-left: .5em;

      &:first-of-type { padding-left: 0 }
    }
  }

  @media screen and (min-width: 30em) {
    .results {
      padding: 0 1em;
      gap: 1em 2em;
      grid-template-columns: repeat(5, 1fr);
      grid-auto-rows: calc((100vw - 4em) / 5);
    }

    h1 {
      font-size: 1.5em;

      span { text-align: left }

      a {
        width: 1em;
        height: 1.1em;
      }
    }

    h2 { text-align: left }

    header {
      border-bottom: 1px solid rgba(255, 255, 255, .2);
      margin: 1em;
    }

    img { object-fit: contain }
  }

  @media screen and (min-width: 50em) {
    .results {
      grid-template-columns: repeat(6, 1fr);
      grid-auto-rows: calc((100vw - 5em) / 6)
    }
  }

  @media screen and (min-width: 85em) {
    .results {
      grid-template-columns: repeat(8, 1fr);
      grid-auto-rows: calc((100vw - 7em) / 8);
    }
  }
</style>

<script lang="ts">
  import { isVisible } from "../utils/viewport";
  import { scale, fly } from "svelte/transition";
  import { onMount, tick } from "svelte";
  import { getAlbum } from "../gql/albums";
  import type { Asset } from "../codegen/types";
  import { operationStore, query } from "@urql/svelte";
  import { querystring, replace } from "svelte-spa-router";
  import Spotlight from "./Spotlight.svelte";

  export let params: { uuid?: string } = {};

  const req = operationStore(getAlbum, { uuid: params.uuid, page: 0 });

  query(req);

  onMount(() => {
    window.scrollTo(0, 0);
    observer.observe(document.getElementById("load-more-photos")!);
  });

  let albumAssets: Array<Asset> = [];
  let hasMore = true;

  $: if (!$req.fetching && $req.data?.album) {
    let add = $req.data.album.assets as Array<Asset>;
    albumAssets = [...albumAssets, ...add];
    if ($req.data.album.assets.length < 10) hasMore = false;
    tick().then(() => onScroll());
  }

  let observer = new IntersectionObserver(onEndOfList, {
    root: null,
    threshold: 0.5
  })

  function onEndOfList(changes: Array<IntersectionObserverEntry>) {
    if (changes[0].isIntersecting) loadMore();
  }

  function loadMore() {
    if (!$req.fetching && hasMore) $req.variables!.page += 1;
  }

  function onScroll() {
    if (isVisible(document.getElementById("load-more-photos"))) loadMore();
  }

  $: assetUUID = $querystring;
  $: album = $req.data?.album;
</script>

{#if assetUUID}
  <Spotlight uuid={assetUUID} />
{/if}

<section class="page">
  {#if album}
    <header transition:fly="{{ x: -40, duration: 400 }}">
      <h1>
        <a href="/#/" title="Back to albums">
          <svg><use xlink:href="#i-chevron-left"/></svg>
        </a>
        <span>{album.title}</span>
      </h1>
      <h2>
        {#if album.photosCount > 0}
          <span>{album.photosCount.toLocaleString()} {album.photosCount > 1 ? 'Photos' : 'Photo'}</span>
        {/if}
        {#if album.videosCount > 0}
          <span>{album.videosCount.toLocaleString()} {album.videosCount > 1 ? 'Videos' : 'Video'}</span>
        {/if}
      </h2>
    </header>
  {/if}

  <div class="results">
    {#each albumAssets as asset (asset.uuid)}
      <figure transition:scale="{{ duration: 350 }}">
        <a href="/#/album/{album.uuid}?{asset.uuid}">
          <img src="http://192.168.1.2:1234/asset/thumb/{asset.uuid}" alt="{asset.uuid}">
        </a>
      </figure>
    {/each}
  </div>

  <div class="load-more" id="load-more-photos" transition:scale="{{ duration: 250 }}">
    {#if $req.fetching}
      <p>💭</p>
    {:else if $req.error}
      <p class="error">
        😵
        {$req.error?.message}
      </p>
    {:else if !hasMore}
      <p>🥳</p>
    {/if}
  </div>
</section>