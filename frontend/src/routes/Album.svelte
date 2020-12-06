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
        stroke-width: 3;
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

        svg { stroke-width: 2 }
      }
    }

    h2 { text-align: left }

    header {
      border-bottom: 1px solid rgba(255, 255, 255, .2);
      margin: 1em;
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
  import { onMount, tick } from "svelte";
  import { getAlbum } from "../gql/albums";
  import { operationStore, query } from "@urql/svelte";
  import type { Asset, Album } from "../codegen/types";
  import { querystring } from "svelte-spa-router";
  import Spotlight from "./Spotlight.svelte";
  import ImageLoader from "../components/ImageLoader.svelte";
  import { Gallery } from "../lib/gallery";

  export let params: { uuid?: string } = {};

  let infiniteScroll: HTMLElement;
  let page = 0;
  let gallery = new Gallery();
  let hasMore = true;

  // Decide on page size at init by working out the
  // optimal number of items based on screen size
  const perPage = 10;

  const req = operationStore(getAlbum, {
    uuid: params.uuid,
    offset: 0,
    limit: perPage,
  });

  query(req);

  const unsubscribe = req.subscribe(value => {
    let fetched = value.data?.album?.assets as Asset[];
    let album = value.data?.album as Album;
    if (album) {
      gallery = gallery.append(fetched);
      if (gallery.size() >= album.photosCount + album.videosCount || !fetched.length) {
        hasMore = false;
      }
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

  $: $req.variables!.offset = page * perPage;
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
    if (!$req.fetching && hasMore) page += 1;
  }
</script>

{#if album && !gallery.isEmpty() && index >= 0}
  <Spotlight {album} {index} {perPage}/>
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

    <div class="results">
      {#each gallery.items as item, i (item.uuid)}
        <figure transition:scale="{{ duration: 350 }}">
          <a href="/#/album/{album.uuid}?{i}">
            <ImageLoader uuid={item.uuid} variant="thumb" />
          </a>
        </figure>
      {/each}
    </div>
  {/if}

  <div class="load-more" bind:this={infiniteScroll} transition:scale="{{ duration: 250 }}">
    {#if $req.fetching}
      <p>💭</p>
    {:else if $req.error}
      <p class="error">
        😵 {$req.error?.message}
      </p>
    {:else if !hasMore}
      <p>🥳</p>
    {/if}
  </div>
</section>