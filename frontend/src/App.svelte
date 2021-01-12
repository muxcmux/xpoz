<style global>
  @import "./styles/index.scss";
</style>

<script>
import Router from "svelte-spa-router";
import { initClient, dedupExchange, fetchExchange } from '@urql/svelte';
import { cacheExchange } from '@urql/exchange-graphcache';
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

const updates = {
  Mutation: {
    deleteToken: (_result: any, args: any, cache: any, _info: any) => {
      cache.invalidate({
        __typename: "Token",
        id: args.id,
      });
    },
  }
}

initClient({
  url: "/api",
  exchanges: [
    dedupExchange,
    cacheExchange({ updates }),
    fetchExchange
  ]
});

setVh();

</script>

<Router {routes}/>
