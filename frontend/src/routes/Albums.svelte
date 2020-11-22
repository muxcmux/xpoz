<style lang="scss">
  p {
    font-size: 1.6em;
    text-align: center;
    font-weight: 300;
    margin: 0;
    height: calc(var(--vh, 1vh) * 100);
    padding: 2em;
    display: grid;
    place-items: center;

    &.error {
      color: var(--color-red);
    }
  }

  section {
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

<script>
  import { fade, scale } from "svelte/transition";
  import { getMyAlbums } from "../gql/albums";
  import type { Album } from "../codegen/types";
  import { operationStore, query } from '@urql/svelte';

  const request = operationStore(getMyAlbums);

  query(request);

  $: loading = $request.fetching as boolean;
  $: error   = $request.error;
  $: albums  = $request.data?.myAlbums as Array<Album>;

</script>

{#if loading}
  <p transition:fade="{{ duration: 150 }}">Loading albums, please wait...</p>
{:else if error}
  <p transition:fade="{{ duration: 150 }}" class="error">
    There was an error loading albums:<br>
    {error.message}
  </p>
{:else if albums}
  <section class="page">
    {#each albums as album, i}
      <a href="#/album/{album.uuid}"
       transition:scale="{{ duration: 250, delay: (250 / albums.length) * i}}">
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
  </section>
{/if}
