<script lang="ts">
  import type { Snippet } from "svelte";
  import type { PluckedButtonProps } from "./Combobox.types";
  import Button from "../Button.svelte";
  import { ChevronDownIcon } from "@lucide/svelte";
  import { twMerge } from "tailwind-merge";

  interface Props extends PluckedButtonProps {
    children: Snippet;
    icon: Snippet;
  }

  const {
    children,
    variant,
    icon,
    class: _class,
    size,
  }: Partial<Props> = $props();
</script>

<div role="group" data-combobox-button="" class="flex items-center">
  <Button
    {variant}
    {size}
    class={twMerge(
      "h-full border-r-0 rounded-tr-none rounded-br-none",
      icon ? "inline-flex" : "block",
      _class as string,
    )}
  >
    {#snippet leftSlot()}
      {#if icon}
        {@render icon()}
      {/if}
    {/snippet}
    {@render children?.()}
  </Button>
  <Button
    icon
    {variant}
    {size}
    class={twMerge(
      "block h-full rounded-tl-none rounded-bl-none",
      size !== "big" ? "" : "p-1.5",
    )}
  >
    <ChevronDownIcon size={size !== "big" ? 16 : 20} />
  </Button>
</div>
