<style global>
  @import "./styles/index.scss";
</style>

<script>
import { onMount } from "svelte";
import { getAllAlbums } from "./gql/albums";

async function query(gql: TemplateStringsArray | string) {
  const result = await fetch("http://localhost:1234/api", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      query: gql
    })
  });
  const json = await result.json();
  return json;
}

const albums: Albums = [];

onMount(async () => {
  try {
    const result = await query(getAllAlbums);
    debugger;
    console.log("RESULT", result);
  } catch (error) {
    console.log("failed with", error);
  }
})
</script>

<h1>List of albums</h1>
<ul>
  {#each albums as album}
    <li>{album.title}</li>
  {:else}
    Loading albums...
  {/each}
</ul>
