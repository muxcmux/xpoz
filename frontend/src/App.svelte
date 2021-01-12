<style global>
  @import "./styles/index.scss";
</style>

<script>
import Router from "svelte-spa-router";
import { initClient } from '@urql/svelte';
import setVh from './utils/vh_fix';
import Albums from "./routes/Albums.svelte";
import Album from "./routes/Album.svelte";
import AccessTokens from "./routes/AccessTokens.svelte";

const routes = {
  '/': Albums,
  '/album/:id': Album,
  '/album/:id/*': Album,
  '/access': AccessTokens
}

initClient({ url: "/api" });

setVh();

</script>

<Router {routes}/>
