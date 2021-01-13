<script lang="ts">
  import { fly } from "svelte/transition";
  import { quartOut } from "svelte/easing";
  import { createEventDispatcher, onMount } from "svelte";
  import type { Token } from "../codegen/types";

  export let token: Token | null;

  let name;

  onMount(() => name.focus());

  const dispatch = createEventDispatcher();
</script>

<div transition:fly={{ easing: quartOut, duration: 400 }} class="backdrop" on:click|preventDefault={() => { dispatch("close") }}></div>

<div transition:fly={{ duration: 400, easing: quartOut, y: 60 }} class="popup">
  <div class="page">
    <header>
      <h1>
        <a href="#" on:click|preventDefault={() => { dispatch("close") }} title="Close">
          Close
        </a>
        <span>{token ? `Edit ${token.name}` : "Create a new token"}</span>
        <i>&nbsp;</i>
      </h1>
      <h2>&nbsp;</h2>
    </header>

    <ul>
      <li>
        <input type="text" placeholder="Name" bind:this={name} />
      </li>
      <li>
        <label>
          <input type="checkbox" /> Admin?
        </label>
      </li>
      <li>
        <label>
          <input type="checkbox" /> Bind to session
        </label>
      </li>
    </ul>
  </div>
</div>

<style lang="scss">
.popup {
  position: absolute;
  bottom: 0;
  top: 3.8em;
  left: 50%;
  transform: translateX(-50%);
  overflow-y: scroll;
  background: var(--color-bg);
  border-radius: 1em 1em 0 0;

  max-width: 40em;
  width: 100vw;
  z-index: 999;
}

.backdrop {
  position: absolute;
  bottom: 0;
  top: 0;
  left: 0;
  right: 0;
  z-index: 998;
  background: rgba(0, 0, 0, .3);
}

input[type=text] {
  background: none;
  border: none;
  box-shadow: none;
  appearance: none;
  color: var(--color-fg);

  &:active, &:focus {
    outline: none;
  }
}

input[type=checkbox] {
  margin-right: .5em;
}

ul {
  list-style: none;
  padding: 0 0 5em 1.5em;
  margin: 1em 0;
  overflow: hidden;
  position: relative;

  li {
    padding: .8em 0;
    font-weight: 600;
    border-top: 1px solid rgba(255, 255, 255, .15);
    position: relative;

    &:last-of-type {
      border-bottom: 1px solid rgba(255, 255, 255, .15);
    }
  }
}
</style>
