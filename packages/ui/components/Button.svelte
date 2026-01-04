<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";
  import { cva } from "class-variance-authority";
  import { twMerge } from "tailwind-merge";

  interface Props extends HTMLButtonAttributes {
    children?: Snippet;

    leftSlot?: Snippet;
    rightSlot?: Snippet;

    icon?: boolean;

    size?: "small" | "big";
    variant?: "primary" | "secondary";
  }

  const {
    class: className,
    icon,
    children,
    size,
    variant,
    leftSlot,
    rightSlot,
    ...others
  }: Props = $props();

  const button = cva(
    twMerge(
      "rounded-sm inline-flex items-center gap-x-1.5 cursor-pointer transition-colors",
      className as string,
    ),
    {
      variants: {
        variant: {
          primary: "border border-red-500 hover:bg-red-900",
          secondary: "hover:bg-red-500/40",
        },
        size: {
          small: !icon ? "px-2 py-1" : "p-1.5",
          big: !icon ? "px-2.5 py-1.5" : "p-1",
        },
      },
    },
  );
</script>

<button
  {...others}
  class={button({
    variant: variant || "primary",
    size: size || "small",
  })}
>
  {#if leftSlot}
    {@render leftSlot?.()}
  {/if}
  {@render children?.()}
  {#if rightSlot}
    {@render rightSlot?.()}
  {/if}
</button>
