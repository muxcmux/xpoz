<style lang="scss">
.transition {
  transition: .4s transform cubic-bezier(0.215, 0.610, 0.355, 1);
}

.fast-transition {
  transition: .15s transform cubic-bezier(0.215, 0.610, 0.355, 1);
}

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

    &.go {
      width: 10px;
      height: 10px;
      opacity: .5;
      stroke-width: 3;
    }
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
}

.touch-controls {
  z-index: 4;
  display: none;

  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;

  div {
    height: 100%;
    display: flex;
    flex-direction: column;
    font-size: .8em;
    font-weight: normal;
    align-items: center;
    justify-content: center;
    width: 70px;
    color: var(--color-bg);

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

      > .tap-delete { z-index: 10 }
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

svg.admin, svg.lock, svg.unlock {
  position: absolute;
  width: 16px;
  height: 16px;
  color: var(--color-darker-orange);
  top: .7em;
  left: -1.2em;
  stroke-width: 2;

}

svg.lock { color: var(--color-red) }
svg.unlock { color: var(--color-green) }

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
  stroke-width: 2;

  svg {
    width: 16px;
    height: 16px;
    margin-right: .5em;
  }
}
</style>

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import * as timeago from "timeago.js";
  import type { Token } from "../codegen/types";

  export let dragging: boolean;
  export let copied: boolean;
  export let x: number;
  export let token: Token;
  export let touchActionWidth: number;
  export let confirmDeleteThreshold: number;

  const dispatch = createEventDispatcher();

  $: firstActionX = touchActionWidth * 2 + x;

  $: secondActionX = touchActionWidth + x / 2;

  $: deleteX = x < confirmDeleteThreshold ? secondActionX - touchActionWidth - 5 : 0;

</script>

<div class="touch-controls">
  {#if copied}
    <div class="tap-tick"
         class:transition={!dragging}
         style="transform: translate3d({firstActionX}px, 0, 0)">
      <svg><use xlink:href="#i-checkmark"/></svg>
      <small>Copied!</small>
    </div>
  {:else}
    <div class="tap-copy"
         class:transition={!dragging}
         on:click|preventDefault|stopPropagation={() => dispatch("copy", {token})}
         style="transform: translate3d({firstActionX}px, 0, 0)">
      <svg><use xlink:href="#i-link"/></svg>
      <small>Share</small>
    </div>
  {/if}
  <div class="tap-delete"
       class:transition={!dragging}
       on:click|preventDefault|stopPropagation={() => dispatch("delete", {token})}
       style="transform: translate3d({secondActionX}px, 0, 0)">
    <div class="tap-delete fast-transition"
         style="transform: translate3d({deleteX}px, 0, 0)">
      <svg><use xlink:href="#i-trash"/></svg>
      <small>Delete</small>
    </div>
  </div>
</div>

<div class="reveal"
     class:transition={!dragging}
     style="transform: translate3d({x}px, 0, 0)">

  {#if token.admin}
    <svg class="admin"><use xlink:href="#i-flag"/></svg>
  {/if}

  {#if token.sessionBound}
    {#if token.sessionId}
      <svg class="lock"><use xlink:href="#i-lock"/></svg>
    {:else}
      <svg class="unlock"><use xlink:href="#i-unlock"/></svg>
    {/if}
  {/if}

  <div class="row">
    <div class="left">{token.name}</div>
    <div class="right row">
      <time>{timeago.format(token.createdAt)}</time>
      <svg class="go"><use xlink:href="#i-chevron-right"/></svg>
    </div>
  </div>

  <div class="row">
    <div class="left">
      <span>
        <svg><use xlink:href="#i-picture"/></svg> {token.whitelistedAlbums == null ? "All" : token.whitelistedAlbums.length}
        | {token.id}
      </span>
    </div>

    <div class="right">
      {#if copied}
        <svg class="tick"><use xlink:href="#i-checkmark"/></svg>
      {:else}
        <svg class="copy" on:click|preventDefault|stopPropagation={() => dispatch("copy", {token})}><use xlink:href="#i-link"/></svg>
      {/if}
      <svg class="delete" on:click|preventDefault|stopPropagation={() => { if (confirm("Are you sure?")) dispatch("delete", {token})} }><use xlink:href="#i-trash"/></svg>
    </div>
  </div>
</div>

