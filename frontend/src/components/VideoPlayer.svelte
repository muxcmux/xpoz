<style lang="scss">
  .video {
    height: 100%;
  }

  video {
    object-fit: cover;
    width: 100%;
    height: 100%;
    background-color: rgba(255, 255, 255, 0);
    transition: .2s background-color ease;
    position: relative;

    @media screen and (min-width: 30em) {
      object-fit: contain;
    }
  }

  .controls {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, .25);
    cursor: pointer;

    .icon {
      width: 7em;
      height: 7em;
      border: 3px solid rgba(255, 255, 255, .4);
      border-radius: 3.5em;
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      color: white;
      display: grid;
      place-content: center;

      span {
        font-size: 4em;
      }

      svg {
        width: 4em;
        height: 4em;
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-47%, -50%);
        stroke-width: 2;
        fill: white;
      }
    }
  }
</style>

<script lang="ts">
  import { fade } from "svelte/transition";

  export let id: string;
  export let paused: boolean;

  let video: HTMLVideoElement;

  let playing = false;
  let ended = false;
  let errored = false;

  $: if (paused && video) { video.pause() }

  function play() {
    playing = true;
  }

  function pause() {
    playing = false;
    ended = false;
  }

  function end() {
    playing = false;
    ended = true;
  }

  function toggle() {
    video.paused ? video.play() : video.pause();
  }

  function error() {
    errored = true;
  }
</script>

<div class="video">
  <video
    on:ended={end}
    on:play={play}
    on:pause={pause}
    on:error={error}
    on:click={toggle}
    bind:this={video}
    playsinline
    preload
    poster="/asset/thumb/{id}"
    src="/asset/video/{id}">
  </video>

  {#if !playing}
    <div class="controls" on:click={toggle} transition:fade={{ duration: 200 }}>
      <div class="icon">
        {#if errored}
          <span>😵</span>
        {:else}
          <svg><use xlink:href="#i-{ended ? 'reload' : 'play'}"/></svg>
        {/if}
      </div>
    </div>
  {/if}
</div>

