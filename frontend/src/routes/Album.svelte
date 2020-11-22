<style lang="scss">
  div {
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
</style>

<script>
  import { fade, scale } from "svelte/transition";
  import { getAlbum } from "../gql/albums";
  import type { Album } from "../codegen/types";
  import { operationStore, query } from '@urql/svelte';

  export let params: { uuid?: String } = {};

  const request = operationStore(getAlbum, { uuid: params.uuid });

  query(request);

  $: loading = $request.fetching as boolean;
  $: error   = $request.error;
  $: album   = $request.data?.album as Album | null;

</script>

<section class="page">
  <h1>You are looking at an album</h1>

  {#if loading}
    <p transition:fade="{{ duration: 150 }}">Loading album, please wait...</p>
  {:else if error}
    <p transition:fade="{{ duration: 150 }}" class="error">
      There was an error loading the album:<br>
      {error.message}
    </p>
  {:else if album}
    <div>
      {#each album.assets as asset}
        <figure>
          <img src="http://localhost:1234/asset/thumb/{asset.uuid}" alt="{asset.uuid}">
        </figure>
      {/each}
    </div>
  {/if}
</section>
