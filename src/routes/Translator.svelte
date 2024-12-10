<script lang="ts">
  import CopyPre from '$lib/CopyPre.svelte';
  import type { TranslateResponseChunk } from '$lib/types';
  import { Channel, invoke } from '@tauri-apps/api/core';
  import { emit } from '@tauri-apps/api/event';
  import { marked } from 'marked';
  import type { EventHandler } from 'svelte/elements';
  import { twMerge } from 'tailwind-merge';

  let { model, class: className }: { model: string; class?: string } = $props();

  let inputFormat: string = $state('');
  let input: string = $state('');
  let outputFormat: string = $state('');

  type State =
    | {
        tag: 'loading';
      }
    | {
        tag: 'streaming';
        response: string;
      }
    | {
        tag: 'done';
        response: string;
      }
    | {
        tag: 'error';
        error: string;
      };

  let responseState: State | undefined = $state.raw();

  const submit: EventHandler<SubmitEvent, HTMLFormElement> = async (e) => {
    e.preventDefault();
    if (
      responseState === undefined ||
      (responseState.tag !== 'streaming' && responseState.tag !== 'loading')
    ) {
      await translate();
    } else {
      stop();
    }
  };

  const translate = async () => {
    responseState = undefined;
    const responseStream = new Channel<TranslateResponseChunk>();
    responseStream.onmessage = (message) => {
      console.log(message);
      if (responseState === undefined) {
        responseState =
          message.tag === 'endOfStream'
            ? undefined
            : {
                tag: message.done ? 'done' : 'streaming',
                response: message.response
              };
      } else {
        responseState = {
          tag: message.tag === 'endOfStream' || message.done ? 'done' : 'streaming',
          response:
            message.tag === 'response'
              ? responseState.tag !== 'error' && responseState.tag !== 'loading'
                ? responseState.response + message.response
                : message.response
              : responseState.tag !== 'error' && responseState.tag !== 'loading'
                ? responseState.response
                : ''
        };
      }
    };

    try {
      const translateResult = invoke('translate', {
        inputFormat,
        input,
        outputFormat,
        model,
        responseStream
      });
      responseState = { tag: 'loading' };
      await translateResult;
    } catch (e) {
      responseState = { tag: 'error', error: new String(e).toString() };
    }
  };

  const stop = () => {
    console.log('stop');
    emit('stop-translation');
  };

  const mdHtml = $derived(
    responseState !== undefined &&
      (responseState.tag === 'done' || responseState.tag === 'streaming')
      ? (marked(responseState.response) as string)
      : undefined
  );

  const domParser = new DOMParser();
  const document = $derived(
    mdHtml
      ? domParser.parseFromString(mdHtml, 'text/html').getElementsByTagName('body').item(0)!
      : undefined
  );
  $inspect(document);
</script>

<div class={twMerge('flex w-max flex-col items-center', className)}>
  <form class="flex w-full flex-col items-center gap-6" onsubmit={submit}>
    <div class="join join-vertical">
      <input
        class="input join-item input-bordered"
        type="text"
        placeholder="Input Format (Optional)"
        bind:value={inputFormat}
      />
      <input
        class="input join-item input-bordered"
        type="text"
        required
        placeholder="Output Format"
        bind:value={outputFormat}
      />
      <textarea
        class="sizing-content join-item textarea textarea-bordered textarea-md"
        required
        placeholder="Input"
        bind:value={input}
      ></textarea>
    </div>

    <button class="btn btn-info btn-md" type="submit">
      {#if responseState === undefined || (responseState.tag !== 'loading' && responseState.tag !== 'streaming')}
        Translate
      {:else}
        <span class="loading loading-dots loading-sm"></span>Stop
      {/if}
    </button>
  </form>

  <article class="prose">
    {#if document}
      {#each document.children as child}
        {#if child.tagName === 'PRE'}
          <CopyPre>
            {@html child.innerHTML}
          </CopyPre>
        {:else}
          {@html child.outerHTML}
        {/if}
      {/each}
    {/if}
  </article>
</div>

<style>
  textarea.sizing-content {
    field-sizing: content;
  }
</style>
