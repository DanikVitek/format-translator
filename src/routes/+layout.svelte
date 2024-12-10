<script lang="ts">
  import '../app.css';
  import Titlebar from '../lib/Titlebar.svelte';
  let { children } = $props();
</script>

<Titlebar />

<div class="h-[calc(100vh-1.75rem)] mt-7 w-full">
  {@render children()}
</div>

<style lang="postcss">
  @global {
    .hide-scrollbar::-webkit-scrollbar {
      display: none;
    }

    .hide-scrollbar {
      -ms-overflow-style: none;
      scrollbar-width: none;
    }
  }
</style>
