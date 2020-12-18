<style global>
  @import "./styles/index.scss";
</style>

<script>
import Router from "svelte-spa-router";
import { initClient } from '@urql/svelte';
import setVh from './utils/vh_fix';
import Albums from "./routes/Albums.svelte";
import Album from "./routes/Album.svelte";

const routes = {
  '/': Albums,
  '/album/:uuid': Album,
  '/album/:uuid/*': Album
}

initClient({ url: "/api" });
setVh();

</script>

<Router {routes}/>
