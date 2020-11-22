import Albums from "./Albums.svelte";
import Album from "./Album.svelte";

export default {
  '/': Albums,
  '/album/:uuid': Album
}