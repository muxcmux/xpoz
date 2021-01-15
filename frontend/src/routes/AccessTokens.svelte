<style lang="scss">
ul {
  list-style: none;
  padding: 0 0 5em 1.5em;
  margin: 1em 0;
  overflow: hidden;
  position: relative;

  li {
    padding: 0;
    font-weight: 600;
    border-bottom: 1px solid rgba(255, 255, 255, .15);
    position: relative;

    &::before {
      content: "";
      position: absolute;
      top: 0;
      left: -50px;
      bottom: 0;
      width: 50px;
      background: var(--color-bg);
      z-index: 1;
    }
  }
}

.backdrop {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  background: black;
}

.page {
  background: var(--color-bg);
  min-height: calc(var(--vh, 1vh) * 100);
  height: 100%;
  transition: transform .4s cubic-bezier(0.165, 0.84, 0.44, 1), border-radius .4s cubic-bezier(0.165, 0.84, 0.44, 1);

  &.hide {
    transform: scale(.92);
    border-radius: 1.2em;
  }
}

#token-link {
  position: absolute;
  left: -9999px;
}
</style>

<script lang="ts">
  import { onDestroy, tick } from "svelte";
  import { fly } from "svelte/transition";
  import { quartOut } from "svelte/easing";
  import { flip } from "svelte/animate";
  import { getTokens, removeToken } from "./../gql/tokens";
  import { mutation, operationStore, query } from "@urql/svelte";
  import type { Token, TokenInput } from "../codegen/types";
  import TokenRow from "../components/TokenRow.svelte";
  import TokenForm from "../components/TokenForm.svelte";
  import fixtap from "../use/fixtap";

  const request = operationStore(getTokens);
  // @ts-ignore
  const deleteMutation = mutation({ query: removeToken });

  let tokens: Token[] = [];
  let showForm = false;
  let selectedToken = tokenToInput(null);

  query(request);

  const unsubscribe = request.subscribe(value => {
    let fetched = value.data?.tokens as Token[];
    if (!value.fetching && fetched) tokens = fetched;
  });

  onDestroy(unsubscribe);

  $: confirmDeleteThreshold = -(viewportWidth * 4/5);

  let viewportWidth: number;
  let touchActionWidth = 70;
  let revealThreshold = -30;

  let copied: string = "";
  let copyTimeout: number;

  function tokenToInput(t: Token | null): TokenInput {
    if (t) {
      return {
        name: t.name,
        admin: t.admin,
        sessionBound: t.sessionBound,
        albumIds: t.whitelistedAlbums?.map(a => a.id) || null,
      }
    }
    return {
      name: "",
      admin: false,
      sessionBound: false,
      albumIds: null
    }
  }

  function deleteToken(id: string) {
    moveX[id] = -viewportWidth;
    tick().then(() => {
      deleteMutation({ id: id });
      tokens = tokens.filter(t => t.id != id);
    })
  }

  function onDelete(e: CustomEvent) {
    deleteToken(e.detail.token.id);
  }

  function onCopy(e: CustomEvent) {
    let token = e.detail.token;
    clearTimeout(copyTimeout);
    let input = document.getElementById('token-link') as HTMLInputElement;
    let link = `${document.location.origin}/auth?${token.id}`;
    input.value = link;
    input.select();
    document.execCommand('copy');
    copied = token.id;
    copyTimeout = setTimeout(() => {
      copied = "";
      moveX[token.id] = 0;
      lastX[token.id] = 0;
    }, 1e3);
  }

  let moveX: {[key: string]: number} = {};
  let lastX: {[key: string]: number} = {};
  let swiping = false;
  let editTokenId: string | null = null;
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
        if (e.detail.deltaX < confirmDeleteThreshold) {
          deleteToken(currentlyMoving!);
        } else {
          moveX[currentlyMoving!] = -(2 * touchActionWidth);
        }
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

  function slideUp(_node: Element, _opts: {}) {
    return {
      duration: 400,
      css: (t: number) => `transform: translate3d(0, -${quartOut(1 - t) * 99}%, 0)`,
    }
  }

  function editToken(t: Token | null) {
    selectedToken = tokenToInput(t);
    showForm = !showForm;
    editTokenId = t?.id || null;
  }
</script>

<svelte:window bind:innerWidth={viewportWidth} />

<div class="backdrop"></div>

<section class="page" class:hide={showForm}>
  <header in:fly="{{ x: -40, duration: 400 }}">
    <h1>
      <a href="/#/" title="Back to albums">
        <svg><use xlink:href="#i-chevron-left"/></svg>
      </a>
      <span>Access Tokens</span>
      <a href="/#/" on:click|preventDefault={() => editToken(null)} title="Add token">
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
      {#each tokens as token (token.id)}
        <li out:slideUp|local animate:flip|local={{ duration: 400, easing: quartOut }}>
          <div use:fixtap on:click={() => editToken(token)}
                          on:touchmove={touchMove}
                          on:panstart={(e) => startRevealing(e, token.id)}
                          on:panmove={reveal}
                          on:panend={stopRevealing}>
            <TokenRow x={moveX[token.id] || 0}
              copied={copied == token.id}
              dragging={currentlyMoving == token.id}
              {token}
              {confirmDeleteThreshold}
              {touchActionWidth}
              on:copy={onCopy}
              on:delete={onDelete} />
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</section>

{#if showForm}
  <TokenForm id={editTokenId} on:close={() => showForm = !showForm} tokenInput={selectedToken} />
{/if}
