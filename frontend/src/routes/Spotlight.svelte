<style lang="scss">
  .backdrop {
    background: black;
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: 5;

    img {
      object-fit: contain;
      width: 100%;
      height: 100%;
    }
  }

  button {
    appearance: none;
    border: none;
    background: rgba(255, 255, 255, .15);
    position: absolute;
    top: 50%;
    transform: translate(0, -50%);
    width: 3em;
    height: 3em;
    border-radius: .5em;
    padding: .8em;
    color: rgba(255, 255, 255, .5);
    outline: none;

    svg {
      width: 100%;
      height: 100%;
      stroke-width: 2;
    }

    &.prev {
      left: 1em;
    }

    &.next {
      right: 1em;
    }
  }
</style>

<script lang="ts">
  import { fade } from "svelte/transition";
  import { pop, location, replace } from "svelte-spa-router";
  import type { Asset } from "src/codegen/types";
  export let index: number;
  export let items: Array<Asset>;

  function next() {
    if (hasNext) { replace(`${$location}?${index + 1}`) }
  }

  function prev() {
    if (hasPrev) { replace(`${$location}?${index - 1}`) }
  }

  $: hasPrev   = index > 0;
  $: hasNext   = index < items.length - 1;
  $: console.log("INDEX IS", index);

  $: currAsset = items[index];
  $: prevAsset = hasPrev ? items[index - 1] : null;
  $: nextAsset = hasNext ? items[index + 1] : null;
</script>

{#if currAsset}
  <div class="backdrop" transition:fade={{ duration: 100 }}>
    <img src="http://192.168.1.2:1234/asset/resized/{currAsset.uuid}"
         alt="{currAsset.uuid}"
         on:click={() => pop()}/>

    {#if hasPrev}
      <button class="prev" on:click={prev}>
        <svg><use xlink:href="#i-chevron-left"/></svg>
      </button>
    {/if}

    {#if hasNext}
      <button class="next" on:click={next}>
        <svg><use xlink:href="#i-chevron-right"/></svg>
      </button>
    {/if}
  </div>
{/if}
