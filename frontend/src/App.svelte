<style global>
  @import "./styles/index.scss";
</style>

<script>
import routes from "./routes/routes";
import Router from "svelte-spa-router";
import { initClient } from '@urql/svelte';
import setVh from './utils/vh_fix';

initClient({ url: "http://localhost:1234/api" });
setVh();

</script>

<Router {routes}/>