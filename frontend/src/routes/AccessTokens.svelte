<style lang="scss">
ul {
  list-style: none;
  padding: 1em 0 1em 1.5em;
  margin: 0;
  overflow: hidden;

  li {
    padding: 0;
    font-weight: 600;
    border-bottom: 1px solid rgba(255, 255, 255, .15);
    position: relative;
  }

}

#token-link {
  position: absolute;
  left: -9999px;
}
</style>

<script lang="ts">
  import { fly } from "svelte/transition";
  import { getTokens } from "./../gql/tokens";
  import { operationStore, query } from "@urql/svelte";
  import type { Token } from "../codegen/types";
  import TokenRow from "../components/TokenRow.svelte";
  import fixtap from "../use/fixtap";

  const request = operationStore(getTokens);

  query(request);

  $: tokens = $request.data?.tokens as Token[] | null;
  $: confirmDeleteThreshold = -(viewportWidth * 4/5);

  let viewportWidth: number;
  let touchActionWidth = 70;
  let revealThreshold = -30;

  let copied: string = "";
  let copyTimeout: number;

  function deleteToken(e: CustomEvent) {

  }

  function copyLink(e: CustomEvent) {
    let token = e.detail.token;
    clearTimeout(copyTimeout);
    let input = document.getElementById('token-link') as HTMLInputElement;
    let link = `${document.location.origin}/auth?${token.token}`;
    input.value = link;
    input.select();
    document.execCommand('copy');
    copied = token.token;
    copyTimeout = setTimeout(() => {
      copied = "";
      moveX[token.token] = 0;
      lastX[token.token] = 0;
    }, 1e3);
  }

  let moveX: {[key: string]: number} = {};
  let lastX: {[key: string]: number} = {};
  let swiping = false;
  let currentlyMoving: string | null = null;

  function startRevealing(e: CustomEvent, id: string) {
    swiping = ["panleft", "panright"].includes(e.detail.additionalEvent);
    currentlyMoving = id;
    Object.keys(moveX).forEach(k => {
      if (currentlyMoving != k) {
        moveX[k] = 0;
        lastX[k] = 0;
      }
    });
  }

  function reveal(e: CustomEvent) {
    if (!swiping) return;

    moveX[currentlyMoving!] = (lastX[currentlyMoving!] || 0) + e.detail.deltaX;
  }

  function stopRevealing(e: CustomEvent) {
    if (swiping) {
      if (e.detail.deltaX < revealThreshold) {
        moveX[currentlyMoving!] = -(2 * touchActionWidth);
      } else {
        moveX[currentlyMoving!] = 0;
      }
      lastX[currentlyMoving!] = moveX[currentlyMoving!];
    }

    currentlyMoving = null;
    swiping = false;
  }

  function touchMove(e: Event) {
    if (swiping) e.preventDefault();
  }
</script>

<svelte:window bind:innerWidth={viewportWidth} />

<section class="page">
  <header in:fly="{{ x: -40, duration: 400 }}">
    <h1>
      <a href="/#/" title="Back to albums">
        <svg><use xlink:href="#i-chevron-left"/></svg>
      </a>
      <span>Access Tokens</span>
      <a href="/#/" title="Add token">
        <svg><use xlink:href="#i-plus"/></svg>
      </a>
    </h1>
    <h2>
      {tokens?.length} {tokens?.length == 1 ? 'token' : 'tokens'}
    </h2>
  </header>

  {#if $request.fetching}
    <p>💭</p>
  {:else if $request.error}
    <p class="error">
      😵 {$request.error?.message}
    </p>
  {:else if tokens}
    <input type="text" id="token-link" readonly />

    <ul>
      {#each tokens as token}
        <li use:fixtap on:touchmove={touchMove} on:panstart={(e) => startRevealing(e, token.token)} on:panmove={reveal} on:panend={stopRevealing}>
          <TokenRow x={moveX[token.token] || 0}
                    copied={copied == token.token}
                    dragging={currentlyMoving == token.token}
                    {token}
                    {confirmDeleteThreshold}
                    {touchActionWidth}
                    on:copy={copyLink}
                    on:delete={deleteToken} />
        </li>
      {/each}
    </ul>
  {/if}
</section>

