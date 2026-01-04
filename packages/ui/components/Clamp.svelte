<script lang="ts">
  import type { Snippet } from "svelte";
  import { twMerge } from "tailwind-merge";

  interface Props {
    children?: Snippet;
    as?: keyof HTMLElementTagNameMap;
    class?: string;
    larger?: boolean;
  }

  const {
    as: Element = "div",
    children,
    class: classN,
    larger,
  }: Props = $props();
</script>

<svelte:element
  this={Element}
  data-clamp-container=""
  class={twMerge([
    larger ? "max-w-screen-2xl" : "max-w-7xl",
    "px-4 w-full mx-auto",
    classN,
  ])}
>
  {@render children?.()}
</svelte:element>
