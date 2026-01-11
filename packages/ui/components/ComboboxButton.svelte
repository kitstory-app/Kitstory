<script lang="ts">
  import type { ComponentProps, Snippet } from "svelte";
  import Button from "./Button.svelte";
  import { ChevronDownIcon } from "@lucide/svelte";
  import { twMerge } from "tailwind-merge";

  type PickedButtonProps = Pick<
    ComponentProps<typeof Button>,
    "variant" | "size" | "class"
  >;

  interface Props extends PickedButtonProps {
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
