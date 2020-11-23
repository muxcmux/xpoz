<style lang="scss">
  .results {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(14em, 1fr));
    grid-auto-rows: 14em;
    gap: 1px;

    a {
      position: relative;

      figure {
        margin: 0;
        overflow: hidden;
        position: absolute;
        width: 100%;
        height: 100%;

        img {
          transition: transform .5s cubic-bezier(0.215, 0.610, 0.355, 1);
          object-fit: cover;
          width: 100%;
          height: 100%;
        }

        figcaption {
          transition: background-color .5s cubic-bezier(0.215, 0.610, 0.355, 1);
          background-color: rgba(0, 0, 0, .4);
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
          opacity: .85;

          div {
            font-size: .9rem;
            font-weight: 300;
            display: flex;
            align-items: center;
            opacity: .8;

            svg {
              width: 16px;
              height: 16px;
              fill: white;
              margin: 0 6px 0 12px;

              &:first-of-type { margin-left: 0 }
            }
          }
        }

        &:hover {
          img { transform: scale(1.2) }
          figcaption { background-color: rgba(0, 0, 0, 0) }
        }
      }
    }
  }

  </style>

<script lang="ts">
  import { isVisible } from "../utils/viewport";
  import { scale } from "svelte/transition";
  import { onMount, afterUpdate } from "svelte";
  import { getMyAlbums } from "../gql/albums";
  import type { Album } from "../codegen/types";
  import { operationStore, query } from '@urql/svelte';

  const request = operationStore(getMyAlbums, { page: 0 });

  query(request);

  onMount(() => {
    window.scrollTo(0, 0);
    observer.observe(document.getElementById("load-more")!);
  });

  let albums: Array<Album> = [];
  let hasMore = true;

  $: if (!$request.fetching && $request.data?.myAlbums) {
    if ($request.data?.myAlbums?.length == 0) {
      hasMore = false;
    } else {
      ($request.data?.myAlbums as Array<Album>).forEach((album) => {
        albums = [...albums, album];
      });
      afterUpdate(() => {
        if (isVisible(document.getElementById("load-more"))) loadMore();
      });
    }
  }

  let observer = new IntersectionObserver(onEndOfList, {
    root: null,
    threshold: 0.5
  })

  function onEndOfList(changes: Array<IntersectionObserverEntry>) {
    if (changes[0].isIntersecting) loadMore();
  }

  function loadMore() {
    if (!$request.fetching && hasMore && $request.variables) {
      console.log("HAS MORE", hasMore, "PAGE", $request.variables?.page);
      $request.variables!.page += 1;
    }
  }
</script>

<section class="page">
  <div class="results">
    {#each albums as album (album.uuid)}
      <a href="#/album/{album.uuid}" transition:scale="{{ duration: 250}}">
        <figure>
          {#if album.keyAssets[0]}
            <img src="http://localhost:1234/asset/thumb/{album.keyAssets[0].uuid}" alt="{album.title || "Album title"}">
          {/if}
          <figcaption>
            {album.title}
            <div>
              {#if album.photosCount > 0}
                <svg><use href="#i-camera"/></svg>
                <span>{album.photosCount}</span>
              {/if}
              {#if album.videosCount > 0}
                <svg><use href="#i-video"/></svg>
                <span>{album.videosCount}</span>
              {/if}
            </div>
          </figcaption>
        </figure>
      </a>
    {/each}
  </div>

  <div id="load-more">
    {#if $request.fetching}
      <p>💭</p>
    {:else if $request.error}
      <p class="error">
        😵
        {$request.error?.message}
      </p>
    {:else if !hasMore}
      <p>🥳</p>
    {/if}
  </div>
</section>
