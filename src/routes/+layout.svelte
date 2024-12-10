<script lang="ts">
  import '../app.css';
  import Titlebar from '../lib/Titlebar.svelte';
  let { children } = $props();
</script>

<Titlebar />

<div class="mt-7 h-[calc(100vh-1.75rem)] w-full">
  {@render children()}
</div>
