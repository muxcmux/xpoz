<style lang="scss">
  .backdrop {
    background: black;
    position: absolute;
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
</style>

<script lang="ts">
  import { fade } from "svelte/transition";
  import { pop } from "svelte-spa-router";
  export let uuid: string;
</script>

<div class="backdrop" transition:fade={{ duration: 150 }}>
  <img src="http://192.168.1.2:1234/asset/resized/{uuid}"
       alt="{uuid}"
       on:click={() => pop()}/>
</div>

