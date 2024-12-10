<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';

	const window = getCurrentWindow();
</script>

<div data-tauri-drag-region class="flex max-h-min select-none justify-end bg-base-300">
	<button aria-label="Minimize" onclick={window.minimize}>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="1em"
			height="1em"
			viewBox="0 0 24 24"
			class="btn btn-square btn-ghost btn-xs inline-flex select-none items-center justify-center"
		>
			<path class="fill-current" d="M20 14H4v-4h16" />
		</svg>
	</button>
	<button aria-label="Maximize" onclick={window.toggleMaximize}>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="1em"
			height="1em"
			viewBox="0 0 24 24"
			class="btn btn-square btn-ghost btn-xs inline-flex select-none items-center justify-center"
		>
			<path class="fill-current" d="M4 4h16v16H4zm2 4v10h12V8z" />
		</svg>
	</button>
	<button aria-label="Close" onclick={window.close}>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="1em"
			height="1em"
			viewBox="0 0 24 24"
			class="btn btn-square btn-xs inline-flex select-none items-center justify-center hover:btn-error"
		>
			<path
				class="fill-current"
				d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"
			/>
		</svg>
	</button>
</div>
