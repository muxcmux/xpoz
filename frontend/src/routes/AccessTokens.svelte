<style lang="scss">
.no-transition { transition: none !important }

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

    .row {
      display: flex;
      align-items: center;
    }

    .left {
      margin-right: auto;
    }

    .right {
      svg {
        width: 24px;
        height: 24px;
        stroke-width: 2;
        cursor: pointer;
        margin-left: .3em;
      }
    }

    .tick {
      color: var(--color-green);
    }

    .copy {
      &:hover {
        color: var(--color-turquoise);
      }
    }

    .delete {
      &:hover {
        color: var(--color-red);
      }
    }

    .reveal {
      padding: .5em .5em .5em 0;
      background: var(--color-bg);
      position: relative;
      z-index: 5;
      transition: .4s transform cubic-bezier(0.215, 0.610, 0.355, 1);
    }

    .touch-controls {
      z-index: 4;
      display: none;

      position: absolute;
      top: 0;
      right: 0;
      bottom: 0;

      > div {
        height: 100%;
        display: flex;
        flex-direction: column;
        font-size: .8em;
        font-weight: normal;
        align-items: center;
        justify-content: center;
        width: 70px;
        color: var(--color-bg);
        transition: .4s transform cubic-bezier(0.215, 0.610, 0.355, 1);

        &::before {
          position: absolute;
          top: 0;
          bottom: 0;
          left: 0;
          width: 900px;
          content: "";
          z-index: 1;
        }

        &.tap-delete {
          background: var(--color-red);
          &::before { background: var(--color-red); }
        }

        &.tap-copy {
          background: var(--color-turquoise);
          &::before { background: var(--color-turquoise); }
        }

        &.tap-tick {
          background: var(--color-green);
          &::before { background: var(--color-green); }
        }
      }

      svg {
        width: 24px;
        height: 24px;
        stroke-width: 2;
        z-index: 2;
      }

      small { z-index: 2 }
    }

    @media (hover: none) {
      .tick, .copy, .delete { display: none }
      .touch-controls { display: flex }
    }

    svg.admin {
      position: absolute;
      width: 16px;
      height: 16px;
      color: var(--color-darker-orange);
      top: .8em;
      left: -1.2em;
      stroke-width: 3;
    }

    time {
      opacity: .5;
      font-size: .7em;
    }

    span {
      font-family: monospace;
      opacity: .7;
      display: flex;
      padding-top: .2em;
      font-weight: normal;

      svg {
        width: 16px;
        height: 16px;
        margin-right: .5em;
      }
    }
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
  import * as timeago from "timeago.js";
  import type { Token } from "../codegen/types";
  import fixtap from "../use/fixtap";

  const request = operationStore(getTokens);

  query(request);

  $: tokens = $request.data?.tokens as Token[] | null;
  let touchActionWidth = 70;
  let revealThreshold = -30;
  let confirmDeleteThreshold = -260;

  let copied: string = "";
  let copyTimeout: number;

  function deleteToken(token: Token) {

  }

  function copyLink(token: Token) {
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

  function firstActionX(x: number): number {
    return touchActionWidth * 2 + x;
  }

  function secondActionX(x: number): number {
    if (x < confirmDeleteThreshold) {
      return touchActionWidth + x;
    } else {
      return touchActionWidth + x / 2;
    }
  }

</script>

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
          <div class="touch-controls">
            {#if copied == token.token}
              <div class="tap-tick"
                   class:no-transition={currentlyMoving == token.token}
                   style="transform: translate3d({firstActionX(moveX[token.token] || 0)}px, 0, 0)">
                <svg><use xlink:href="#i-checkmark"/></svg>
                <small>Copied!</small>
              </div>
            {:else}
              <div class="tap-copy"
                   class:no-transition={currentlyMoving == token.token}
                   style="transform: translate3d({firstActionX(moveX[token.token] || 0)}px, 0, 0)">
                <svg on:click|preventDefault={() => copyLink(token)}><use xlink:href="#i-link"/></svg>
                <small>Share</small>
              </div>
            {/if}
            <div class="tap-delete"
                 class:no-transition={currentlyMoving == token.token}
                 style="transform: translate3d({secondActionX(moveX[token.token] || 0)}px, 0, 0)">
              <svg on:click|preventDefault={() => deleteToken(token)}><use xlink:href="#i-trash"/></svg>
              <small>Delete</small>
            </div>
          </div>

          <div class="reveal"
               class:no-transition={currentlyMoving == token.token}
               style="transform: translate3d({moveX[token.token] || 0}px, 0, 0)">

            {#if token.admin}
              <svg class="admin"><use xlink:href="#i-flag"/></svg>
            {/if}

            <div class="row">
              <div class="left">{token.name}</div>
              <time class="right">{timeago.format(token.createdAt)}</time>
            </div>

            <div class="row">
              <div class="left">
                <span>
                  {#if token.sessionBound && token.sessionId}
                    <svg><use xlink:href="#i-lock"/></svg>
                  {/if}

                  {token.token}
                </span>
              </div>

              <div class="right">
                {#if copied == token.token}
                  <svg class="tick"><use xlink:href="#i-checkmark"/></svg>
                {:else}
                  <svg class="copy" on:click|preventDefault={() => copyLink(token)}><use xlink:href="#i-link"/></svg>
                {/if}
                <svg class="delete" on:click|preventDefault={() => { if (confirm("Are you sure?")) deleteToken(token)} }><use xlink:href="#i-trash"/></svg>
              </div>
            </div>
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</section>

