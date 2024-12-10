<script lang="ts">
  import type { MouseEventHandler } from 'svelte/elements';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import hljs from 'highlight.js';

  const { language, text }: { language: string; text: string } = $props();

  const copyCode: MouseEventHandler<HTMLButtonElement> = async (e) => {
    const btn = e.currentTarget;
    await writeText(btn.parentElement!.querySelector('code')!.innerText);
    btn.classList.add('tooltip-open');
    btn.classList.remove('hover:before:opacity-0', 'hover:after:opacity-0');
    setTimeout(() => {
      btn.classList.remove('tooltip-open');
      btn.classList.add('hover:before:opacity-0', 'hover:after:opacity-0');
    }, 1000);
  };

  const hl = $derived(
    language !== '' ? hljs.highlight(text, { language, ignoreIllegals: true }).value : ''
  );
</script>

<div class="relative">
  <button
    class="btn btn-square btn-ghost btn-sm tooltip tooltip-right tooltip-success absolute right-2 top-2 z-10 inline-flex items-center hover:before:opacity-0 hover:after:opacity-0"
    data-tip="Copied!"
    aria-label="Copy code"
    onclick={copyCode}
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
      <path
        class="fill-current"
        d="M15.24 2h-3.894c-1.764 0-3.162 0-4.255.148c-1.126.152-2.037.472-2.755 1.193c-.719.721-1.038 1.636-1.189 2.766C3 7.205 3 8.608 3 10.379v5.838c0 1.508.92 2.8 2.227 3.342c-.067-.91-.067-2.185-.067-3.247v-5.01c0-1.281 0-2.386.118-3.27c.127-.948.413-1.856 1.147-2.593s1.639-1.024 2.583-1.152c.88-.118 1.98-.118 3.257-.118h3.07c1.276 0 2.374 0 3.255.118A3.6 3.6 0 0 0 15.24 2"
      />
      <path
        class="fill-current"
        d="M6.6 11.397c0-2.726 0-4.089.844-4.936c.843-.847 2.2-.847 4.916-.847h2.88c2.715 0 4.073 0 4.917.847S21 8.671 21 11.397v4.82c0 2.726 0 4.089-.843 4.936c-.844.847-2.202.847-4.917.847h-2.88c-2.715 0-4.073 0-4.916-.847c-.844-.847-.844-2.21-.844-4.936z"
      />
    </svg>
  </button>
  <pre class="pr-11"><code class={`language-${language}`}>{@html hl}</code></pre>
</div>
