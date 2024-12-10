<script lang="ts">
	import '../app.css';
	import Titlebar from '../lib/Titlebar.svelte';
	let { children } = $props();
</script>

<Titlebar />

{@render children()}
