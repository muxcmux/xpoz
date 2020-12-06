<style lang="scss">
  .results {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(14em, 1fr));
    grid-auto-rows: 14em;
    gap: 2px;

    a {
      position: relative;

      figure {
        margin: 0;
        overflow: hidden;
        position: absolute;
        width: 100%;
        height: 100%;

        :global(.image-loader img) {
          transition: transform .5s cubic-bezier(0.215, 0.610, 0.355, 1);
          object-fit: cover;
          width: 100%;
          height: 100%;
        }

        figcaption {
          transition: background-color .5s cubic-bezier(0.215, 0.610, 0.355, 1);
          background-color: rgba(0, 0, 0, .5);
          position: absolute;
          z-index: 2;
          color: white;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          padding: 1em;
          display: flex;
          flex-direction: column;
          justify-content: flex-end;
          font-weight: 600;
          font-size: 1.2em;
          opacity: .85;

          div {
            font-size: 1rem;
            font-weight: 300;
            display: flex;
            align-items: center;
            opacity: .8;

            svg {
              width: 1.1em;
              height: 1.1em;
              fill: white;
              stroke-width: 2;
              margin: 0 .3em 0 .8em;

              &:first-of-type { margin-left: 0 }
            }
          }
        }

        &:hover {
          :global(img) { transform: scale(1.2) }
          figcaption { background-color: rgba(0, 0, 0, 0) }
        }
      }
    }
  }

  @media screen and (min-width: 30em) {
    .results {
      gap: 1px;

      a {
        figure {
          figcaption {
            font-size: 1em;
          }
        }
      }
    }
  }
 </style>

<script lang="ts">
  import { isVisible } from "../utils/viewport";
  import { scale } from "svelte/transition";
  import { onMount, tick } from "svelte";
  import { getMyAlbums } from "../gql/albums";
  import type { Album } from "../codegen/types";
  import { operationStore, query } from '@urql/svelte';
  import { Gallery } from "../lib/gallery";
  import ImageLoader from "../components/ImageLoader.svelte";

  let infiniteScroll: HTMLElement;
  const request = operationStore(getMyAlbums, { page: 0 });
  let gallery = new Gallery<Album>();
  let hasMore = true;

  query(request);

  const unsubscribe = request.subscribe(async value => {
    let fetched = value.data?.myAlbums as Album[];
    if (fetched) {
      gallery = gallery.append(fetched);
      if (fetched?.length < 10) hasMore = false;
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

  let observer = new IntersectionObserver(onEndOfList, {
    root: null,
    threshold: 0.5
  })

  function onEndOfList(changes: IntersectionObserverEntry[]) {
    if (changes[0].isIntersecting) loadMore();
  }

  function loadMore() {
    if (!$request.fetching && hasMore) $request.variables!.page += 1;
  }
</script>

<section class="page">
  <div class="results">
    {#each gallery.items as item (item.uuid)}
      <a href="/#/album/{item.asset.uuid}" in:scale="{{ duration: 350}}">
        <figure>
          {#if item.asset.keyAssets[0]}
            <ImageLoader uuid={item.asset.keyAssets[0].uuid} variant="thumb" alt={item.asset.title || "Album title"} />
          {/if}
          <figcaption>
            {item.asset.title}
            <div>
              {#if item.asset.photosCount > 0}
                <svg><use xlink:href="#i-camera"/></svg>
                <span>{item.asset.photosCount.toLocaleString()}</span>
              {/if}
              {#if item.asset.videosCount > 0}
                <svg><use xlink:href="#i-video"/></svg>
                <span>{item.asset.videosCount}</span>
              {/if}
            </div>
          </figcaption>
        </figure>
      </a>
    {/each}
  </div>

  <div class="load-more" bind:this={infiniteScroll}>
    {#if $request.fetching}
      <p>💭</p>
    {:else if $request.error}
      <p class="error">
        😵 {$request.error?.message}
      </p>
    {:else if !hasMore}
      <p>~</p>
    {/if}
  </div>
</section>
