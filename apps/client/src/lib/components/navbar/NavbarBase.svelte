<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children?: Snippet;
  }

  const { children }: Props = $props();
</script>

<nav class="fixed top-0 inset-x-0 flex items-center justify-between px-3 h-(--nav-padding-y)">
  {@render children?.()}
</nav>
