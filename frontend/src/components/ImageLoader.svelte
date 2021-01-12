<style lang="scss">
  .image-loader {
    height: 100%;
  }

  img {
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
</style>

<div class="image-loader {state}">
  {#if src}
    <img {src} {alt} in:fade="{{ duration: 150 }}" />
  {/if}
</div>

<script lang="ts">
  import { onDestroy } from "svelte";
  import { fade } from "svelte/transition";

  type Variant = "original" | "resized" | "render" | "thumb";
  type State = "loading-thumb" | "loading" | "loaded" | "thumb-failed" | "failed";

  export let id: string;
  export let variant: Variant;
  export let alt: string = "";

  const preloaders: {[key: string]: HTMLImageElement | null} = {
    thumb: null,
    image: null
  };

  let state: State = "loading-thumb";
  let source: string | null = null;

  function getImagePath(id: string, variant: string): string {
    const imageServer = "/asset";
    const variants:{[key:string]: string} = {
      original: `${imageServer}/original/${id}`,
      resized: `${imageServer}/resized/${id}`,
      render: `${imageServer}/render/${id}`,
      thumb: `${imageServer}/thumb/${id}`
    }
    return variants[variant];
  }

  onDestroy(() => {
    destroyLoader("thumb");
    destroyLoader("image");
  });

  function destroyLoader(which: string): void {
    if (preloaders[which]) {
      preloaders[which]!.onload = null;
      preloaders[which]!.onerror = null;
      preloaders[which] = null;
      delete preloaders[which];
    }
  }

  function preload(): void {
    destroyLoader("thumb");
    destroyLoader("image");

    source = null;

    preloaders.thumb = new Image();
    state = "loading-thumb";

    preloaders.thumb.onload = () => {
      destroyLoader("thumb");
      if (state != "loaded") {
        source = getImagePath(id, "thumb");
        state = variant == "thumb" ? "loaded" : "loading";
      }
    }

    preloaders.thumb.onerror = () => {
      destroyLoader("thumb");
      if (variant == "thumb") {
        state = "failed";
      } else if (state != "failed" && state != "loaded") {
        state = "thumb-failed";
      }
    }

    preloaders.thumb.src = getImagePath(id, "thumb");

    if (variant != "thumb") {
      preloaders.image = new Image();

      preloaders.image.onload = () => {
        destroyLoader("image");
        source = getImagePath(id, variant);
        state = "loaded";
      }

      preloaders.image.onerror = () => {
        destroyLoader("image");
        state = "failed";
      }

      preloaders.image.src = getImagePath(id, variant);
    }
  }

  $: src = source;
  $: if (id != null) preload()


</script>
