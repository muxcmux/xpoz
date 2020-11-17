<style global>
  @import "./styles/index.scss";
</style>

<script lang="ts">
import { initClient, operationStore, query } from '@urql/svelte';
import { getAllAlbums } from "./gql/albums";
import type { Album } from "./codegen/types";

initClient({ url: "http://localhost:1234/api" });

const request = operationStore(getAllAlbums, undefined, { pause: true });

query(request);

function loadAlbums(): void {
  $request.context!.pause = false;
}

$: loading = !$request.context?.pause as boolean && $request.fetching as boolean;
$: error = $request.error;
$: albums = $request.data?.myAlbums as Array<Album>;

</script>

<div>
  <h1>Здравей</h1>

  <button on:click="{ loadAlbums }">Load some albums</button>


  {#if loading}
    <p>Loading albums, please wait...</p>
  {:else if error}
    <p class="red">
      There was an error loading albums:<br>
      {error.message}
    </p>
  {:else if albums}
    <ul>
      {#each albums as album}
        <li>{album.title} {album.uuid}</li>
      {/each}
    </ul>
  {/if}
</div>
