<script lang="ts">
	import type { GenerationResponse } from '$lib/types';
	import { Channel, invoke } from '@tauri-apps/api/core';
	import { emit } from '@tauri-apps/api/event';
	import type { EventHandler } from 'svelte/elements';

	let { model }: { model: string } = $props();

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
	let error: string | undefined = $state();

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
		const responseStream = new Channel<GenerationResponse>();
		responseStream.onmessage = (message) => {
			console.log(message);
			if (responseState === undefined) {
				responseState = {
					tag: message.done ? 'done' : 'streaming',
					response: message.response
				};
			} else {
				responseState = {
					tag: message.done ? 'done' : 'streaming',
					response:
						responseState.tag !== 'error' && responseState.tag !== 'loading'
							? responseState.response + message.response
							: message.response
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
			error = new String(e).toString();
		}
	};

	const stop = () => {
		console.log('stop');
		emit('stop-translation');
	};
</script>

<div class="flex h-full w-full flex-col items-center">
	<form class="flex w-full flex-col items-center gap-6" onsubmit={submit}>
		<div class="flex w-full flex-row justify-evenly">
			<div class="join join-vertical">
				<input
					class="input join-item input-bordered"
					type="text"
					placeholder="Input Format"
					bind:value={inputFormat}
				/>
				<textarea
					class="sizing-content join-item textarea textarea-bordered"
					required
					placeholder="Input"
					bind:value={input}
				></textarea>
			</div>
			<div class="join join-vertical">
				<input
					class="input join-item input-bordered"
					type="text"
					required
					placeholder="Output Format"
					bind:value={outputFormat}
				/>
				<textarea
					class="sizing-content join-item textarea textarea-bordered"
					readonly
					placeholder="Output"
					value={responseState !== undefined &&
					(responseState.tag === 'done' || responseState.tag === 'streaming')
						? responseState.response
						: undefined}
				></textarea>
			</div>
		</div>
		<button class="btn btn-info btn-md" type="submit">
			{#if responseState === undefined || (responseState.tag !== 'loading' && responseState.tag !== 'streaming')}
				Translate
			{:else}
				<span class="loading loading-dots loading-sm"></span>Stop
			{/if}
		</button>
	</form>
</div>

<style>
	textarea.sizing-content {
		field-sizing: content;
	}
</style>
