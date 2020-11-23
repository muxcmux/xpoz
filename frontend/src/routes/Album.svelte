<style lang="scss">
  .results {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-auto-rows: calc((100vw - 3px) / 4);
    gap: 1px;

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

  header { margin: 1em; }

  h1 {
    margin: 0;
    color: var(--color-fg);
    font-weight: 700;
    font-size: 1.1em;

    svg {
      height: .7em;
      width: .7em;
      margin-right: .2em;
    }
  }

  h2 {
    color: var(--color-fg);
    font-weight: 400;
    font-size: 1em;
    opacity: .5;
    margin-top: .5em;
    margin-bottom: .5em;

    span {
      padding-right: .5em;
    }
  }

  @media screen and (min-width: 30em) {
    .results {
      padding: 0 1em;
      gap: 1em 2em;
      grid-template-columns: repeat(5, 1fr);
      grid-auto-rows: calc((100vw - 4em) / 5);
    }

    h1 { font-size: 1.5em }

    header {
      border-bottom: 1px solid rgba(255, 255, 255, .2);
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
  import { scale } from "svelte/transition";
  import { onMount, tick } from "svelte";
  import { getAlbum } from "../gql/albums";
  import type { Album, Asset } from "../codegen/types";
  import { operationStore, query } from '@urql/svelte';

  export let params: { uuid?: String } = {};

  const req = operationStore(getAlbum, { uuid: params.uuid, page: 0 });

  query(req);

  onMount(() => {
    window.scrollTo(0, 0);
    observer.observe(document.getElementById("load-more")!);
  })

  let assets: Array<Asset> = [];
  let hasMore = true;

  $: if (!$req.fetching && $req.data?.album) {
    ($req.data.album.assets as Array<Asset>).forEach((asset) => {
      assets = [...assets, asset];
    });
    if ($req.data.album.assets.length < 10) hasMore = false;
    tick().then(() => {
      if (isVisible(document.getElementById("load-more"))) loadMore();
    });
  }

  let observer = new IntersectionObserver(onEndOfList, {
    root: null,
    threshold: 0.5
  })

  function onEndOfList(changes: Array<IntersectionObserverEntry>) {
    if (changes[0].isIntersecting) loadMore();
  }

  function loadMore() {
    if (!$req.fetching && hasMore && $req.variables) {
      console.log($req.variables.page);
      $req.variables!.page += 1;
    }
  }
</script>

<section class="page">
  <header>
    <h1>
      <a href="/#/" title="Back to albums">
        <svg><use href="#i-chevron-left"/></svg>
      </a>
      {#if $req.data?.album}
        {$req.data.album.title}
      {:else}
        <span style="opacity: .5">Loading...</span>
      {/if}
    </h1>
    <h2>
      {#if $req.data?.album?.photosCount > 0}
        <span>{$req.data.album.photosCount} {$req.data.album.photosCount > 1 ? 'Photos' : 'Photo'}</span>
      {/if}
      {#if $req.data?.album?.videosCount > 0}
        <span>{$req.data.album.videosCount} {$req.data.album.videosCount > 1 ? 'Videos' : 'Video'}</span>
      {/if}
    </h2>
  </header>

  <div class="results">
    {#each assets as asset (asset.uuid)}
      <figure transition:scale="{{ duration: 250}}">
        <img src="http://localhost:1234/asset/thumb/{asset.uuid}" alt="{asset.uuid}">
      </figure>
    {/each}
  </div>

  <div id="load-more">
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
