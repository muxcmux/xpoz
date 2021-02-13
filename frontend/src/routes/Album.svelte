<style lang="scss">
  .results {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-auto-rows: calc((100vw - 6px) / 4);
    gap: 2px;

    :global(.image-loader) {
      transition: background-color, .2s ease;

      &.loading-thumb {
        background-color: rgba(255, 255, 255, .1);
      }
    }
  }

  figure {
    position: relative;
    margin: 0;
    width: 100%;
    height: 100%;

    a {
      display: block;
      height: 100%;
    }

    .duration {
      color: white;
      padding: 0.05em 0.25em;
      font-size: .8em;
      position: absolute;
      right: 0;
      bottom: 0;
      z-index: 4;
      background: rgba(0, 0, 0, .3);
    }
  }

  @media screen and (min-width: 30em) {
    .results {
      padding: 0 1em;
      gap: 1em 2em;
      grid-template-columns: repeat(5, 1fr);
      grid-auto-rows: calc((100vw - 4em) / 5);
    }
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
  import fixtap from "../use/fixtap";
  import { push } from "svelte-spa-router";
  import { onMount, tick } from "svelte";
  import { getAlbum } from "../gql/albums";
  import { operationStore, query } from "@urql/svelte";
  import type { Asset, Album } from "../codegen/types";
  import { querystring } from "svelte-spa-router";
  import Spotlight from "./Spotlight.svelte";
  import ImageLoader from "../components/ImageLoader.svelte";
  import { Gallery } from "../lib/gallery";

  export let params: { id?: string } = {};

  let infiniteScroll: HTMLElement;
  let gallery = new Gallery();
  let hasMore = true;

  // Decide on page size at init by working out the
  // optimal number of items based on screen size
  const perPage = 10;

  const req = operationStore(getAlbum, {
    id: params.id,
    offset: 0,
    limit: perPage,
  });

  query(req);

  const unsubscribe = req.subscribe(async value => {
    let fetched = value.data?.album?.assets as Asset[];
    let album = value.data?.album as Album;
    if (album) {
      gallery = gallery.append(fetched);
      if (gallery.size() >= album.photosCount + album.videosCount || !fetched.length) {
        hasMore = false;
      }
      await tick();
      if (isVisible(infiniteScroll)) loadMore();
    }
  });

  onMount(() => {
    window.scrollTo(0, 0);
    observer.observe(infiniteScroll);

    return () => {
      observer.unobserve(infiniteScroll);
      unsubscribe();
    }
  });

  $: index = parseInt($querystring!);
  $: album = $req.data?.album;

  let observer = new IntersectionObserver(onEndOfList, {
    root: null,
    threshold: 0.5
  })

  function onEndOfList(changes: Array<IntersectionObserverEntry>) {
    if (changes[0].isIntersecting) loadMore();
  }

  function loadMore() {
    if (!$req.fetching && hasMore) $req.variables!.offset += perPage;
  }

  function duration(seconds: number): string {
    let min = Math.floor(seconds / 60);
    let s = Math.floor(seconds) % 60;
    let sec = s < 10 ? `0${s}` : s;
    return `${min}:${sec}`;
  }
</script>

{#if album && !gallery.isEmpty() && index >= 0}
  <Spotlight {album} {index} {perPage}/>
{/if}

<section class="page">
  {#if album}
    <header in:fly="{{ x: -40, duration: 400 }}">
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

    <div class="results">
      {#each gallery.items as item, i (item.id)}
        <figure in:scale="{{ duration: 350 }}">
          <a href="/#/album/{album.id}?{i}" use:fixtap on:tap={() => push(`/album/${album.id}?${i}`)}>
            <ImageLoader id={item.id} variant="thumb" />
            {#if item.asset.isVideo}
              <div class="duration">{duration(item.asset.duration)}</div>
            {/if}
          </a>
        </figure>
      {/each}
    </div>
  {/if}

  <div class="load-more" bind:this={infiniteScroll}>
    {#if $req.fetching}
      <p>💭</p>
    {:else if $req.error}
      <p class="error">
        😵 {$req.error?.message}
      </p>
    {:else if !hasMore}
      <p>~</p>
    {/if}
  </div>
</section>
