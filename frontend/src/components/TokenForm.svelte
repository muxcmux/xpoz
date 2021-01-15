<script lang="ts">
  import { fly } from "svelte/transition";
  import { quartOut } from "svelte/easing";
  import { createEventDispatcher, onMount } from "svelte";
  import type { TokenInput } from "../codegen/types";
  import { operationStore, query, mutation } from '@urql/svelte';
  import { getMyAlbumsForAccess } from "../gql/albums";
  import { createToken, updateToken } from "../gql/tokens";

  export let tokenInput: TokenInput;
  export let id: string | null;

  let name;
  let whitelist = tokenInput.albumIds != null;
  const originalAlbumIds = tokenInput.albumIds;

  onMount(() => name.focus());

  const dispatch = createEventDispatcher();
  const request = operationStore(getMyAlbumsForAccess);
  const createMutation = mutation({ query: createToken });
  const updateMutation = mutation({ query: updateToken });

  query(request);

  $: albums = $request.data?.myAlbums as Album[];

  function slideUp(_node: Element, _opts: {}) {
    return {
      duration: 400,
      css: (t: number) => `transform: translate3d(0, -${quartOut(1 - t) * 99}%, 0)`,
    }
  }

  function save(e: Event) {
    if (id) {
      updateMutation({ id: id, input: tokenInput });
    } else {
      createMutation({ input: tokenInput });
    }
    dispatch("close");
  }

  function selectAll(e: Event) {
    if (albums) tokenInput.albumIds = albums.map(a => a.id);
  }

  $: tokenInput.albumIds = whitelist ? originalAlbumIds || [] : null;
  $: console.log(tokenInput);

</script>

<div transition:fly={{ easing: quartOut, duration: 400 }} class="backdrop" on:click|preventDefault={() => { dispatch("close") }}></div>

<div transition:fly={{ duration: 400, easing: quartOut, y: 60 }} class="popup">
  <div class="page">
    <header>
      <h1>
        <a href="#" on:click|preventDefault={() => { dispatch("close") }} title="Close">
          Close
        </a>
        <span>{id ? `Edit ${tokenInput.name || "Token"}` : "Create a new token"}</span>
        <a href="#" on:click|preventDefault={save} title="Save">
          {id ? "Save" : "Create"}
        </a>
      </h1>
      <h2>&nbsp;</h2>
    </header>

    <ul>
      <li>
        <input type="text" placeholder="Name" bind:this={name} bind:value={tokenInput.name}/>
      </li>
      <li>
        <input type="checkbox" id="admin" bind:checked={tokenInput.admin}/>
        <label for="admin">Admin</label>
      </li>
      <li>
        <input type="checkbox" id="session" bind:checked={tokenInput.sessionBound}/>
        <label for="session">Bind to session</label>
      </li>
      <li>
        <input type="checkbox" id="whitelist" bind:checked={whitelist}/>
        <label for="whitelist">Whitelist albums</label>
      </li>
      {#if whitelist}
        <li class="heading">ALBUMS TO WHITELIST <a href="#" on:click|preventDefault={selectAll}>Select All</a></li>
        {#each (albums || []) as album}
          <li class="album">
            <input type="checkbox" bind:group={tokenInput.albumIds} id={album.id} value={album.id}/>
            <label for={album.id}>{album.title}</label>
          </li>
        {/each}
      {/if}
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

    &.album {
      font-weight: normal;
    }

    &.heading {
      padding: .4em 1em .4em 0;
      font-size: .8em;
      font-weight: normal;
      background: rgba(255,255,255,.05);
      position: relative;
      display: flex;
      justify-content: space-between;

      a { white-space: nowrap }

      &::before {
        position: absolute;
        background: rgba(255,255,255,.05);
        border-bottom: 1px solid rgba(255, 255, 255, .15);
        border-top: 1px solid rgba(255, 255, 255, .15);
        width: 50px;
        left: -50px;
        content: "";
        top: -1px;
        bottom: -1px;
      }
    }

    input[type=checkbox] {
      opacity: 0;
      position: absolute;

      &:checked + label {
        &::after {
          transform: translateX(20px);
        }
        &::before {
          background-color: var(--color-green);
        }
      }
    }

    label {
      position: relative;
      display: block;
      padding-left: 56px;

      &::before, &::after {
        content: "";
        left: 0;
        top: 0;
        height: 24px;
        border-radius: 12px;
        position: absolute;
      }

      &::before {
        width: 44px;
        background: rgba(255,255,255,.2);
        transition:  background-color .25s ease;
      }

      &::after {
        width: 24px;
        background: white;
        transition: transform .25s ease;
      }
    }
  }
}
</style>
