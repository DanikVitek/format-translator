<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Translator from './Translator.svelte';
  import { type LocalModel } from '$lib/types';
  import CopyPre from './CodeBlock.svelte';

  const DEFAULT_HOST = 'http://localhost:11434';

  type State =
    | {
        tag: 'disconnected';
      }
    | {
        tag: 'connecting';
      }
    | {
        tag: 'connected';
      }
    | {
        tag: 'error';
        error: unknown;
      };

  let host = $state(DEFAULT_HOST);

  let connectionState: State = $state.raw({ tag: 'disconnected' });

  let model = $state<LocalModel>();
</script>

<main class="flex h-full w-full flex-col items-center overflow-auto">
  <form class="mt-4 flex flex-row">
    <input class="input input-bordered" type="text" placeholder="Host" bind:value={host} />
    {#if connectionState.tag === 'connected'}
      <button
        class="btn btn-outline btn-md"
        onclick={async () => {
          await invoke('disconnect');
          connectionState = { tag: 'disconnected' };
          model = undefined;
        }}
      >
        Disconnect
      </button>
    {:else}
      <button
        class="btn btn-info btn-md"
        onclick={async () => {
          try {
            let connectionResult = invoke('connect', { host });
            connectionState = { tag: 'connecting' };
            await connectionResult;
            connectionState = { tag: 'connected' };
          } catch (e) {
            console.error(e);
            connectionState = { tag: 'error', error: e };
          }
        }}
      >
        Connect
      </button>
      {#if connectionState.tag === 'error'}
        <p class="text-error">{connectionState.error}</p>
      {/if}
    {/if}
  </form>
  {#if connectionState.tag === 'connected'}
    {#await invoke<LocalModel[]>('list_models') then models}
      <select class="select select-bordered" bind:value={model} placeholder="Select a model">
        <option selected value={undefined}>Select a model</option>
        {#each models as modelOption}
          <option value={modelOption}>{modelOption.name}</option>
        {/each}
      </select>
    {:catch error}
      <p class="text-error">{error}</p>
    {/await}
    {#if model}
      <Translator model={model.name} />
    {/if}
  {:else if connectionState.tag === 'connecting'}
    <span class="loading loading-dots loading-lg"></span>
  {/if}
</main>
