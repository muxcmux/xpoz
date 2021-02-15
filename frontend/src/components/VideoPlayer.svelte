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
      width: 5em;
      height: 5em;
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      color: white;
      display: grid;
      place-content: center;
      backdrop-filter: blur(4px);
      -webkit-backdrop-filter: blur(4px);
      background: rgba(0, 0, 0, .2);
      border-radius: 2.5em;

      span {
        font-size: 4em;
      }

      svg {
        width: 3em;
        height: 3em;
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-47%, -50%);
        stroke-width: 2;
        fill: white;
      }
    }
  }

  .progress {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: .2em;

    .track {
      position: absolute;
      left: 0;
      top: 0;
      bottom: 0;
      background: rgba(255,255,255,.7);
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
  let buffering = false;

  let progress = 0;

  $: if (paused && video) { video.pause() }

  function play() {
    playing = true;
    buffering = false;
    error = false;
    ended = false;
  }

  function pause() {
    playing = false;
    buffering = false;
    error = false;
    ended = false;
  }

  function end() {
    playing = false;
    buffering = false;
    error = false;
    ended = true;
  }

  function toggle() {
    video.paused ? video.play() : video.pause();
  }

  function error() {
    errored = true;
    buffering = false;
  }

  function wait() {
    buffering = true;
  }

  function timeupdate() {
    progress = video.currentTime / video.duration * 100;
  }

  $: controlsClass = buffering || ended || errored ? 'plain' : '';

  $: if (id && video) {
    video.pause();
    video.load();
    ended = false;
  }
</script>

<div class="video">
  <video
    on:ended={end}
    on:play={play}
    on:pause={pause}
    on:error={error}
    on:click={toggle}
    on:playing={play}
    on:waiting={wait}
    on:timeupdate={timeupdate}
    bind:this={video}
    playsinline
    poster="/asset/thumb/{id}">
    <source src="/asset/video/{id}"/>
    <source src="/asset/original/{id}"/>
  </video>

  <div class="progress">
    <div class="track" style="width: {progress}%"></div>
  </div>

  {#if !playing || buffering}
    <div class="controls" on:click={toggle} transition:fade={{ duration: 200 }}>
      <div class="icon">
        {#if errored}
          <span>😵</span>
        {:else if buffering}
          <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" style="margin: auto; background: transparent; display: block;" width="200px" height="200px" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid">
            <g transform="rotate(0 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.9166666666666666s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(30 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.8333333333333334s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(60 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.75s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(90 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.6666666666666666s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(120 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.5833333333333334s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(150 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.5s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(180 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.4166666666666667s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(210 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.3333333333333333s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(240 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.25s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(270 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.16666666666666666s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(300 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.08333333333333333s" repeatCount="indefinite"></animate>
              </rect>
              </g><g transform="rotate(330 50 50)">
              <rect x="47" y="24" rx="3" ry="6" width="6" height="12" fill="#ffffff">
                <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="0s" repeatCount="indefinite"></animate>
              </rect>
            </g>
          </svg>
        {:else}
          <svg><use xlink:href="#i-{ended ? 'reload' : 'play'}"/></svg>
        {/if}
      </div>
    </div>
  {/if}
</div>

