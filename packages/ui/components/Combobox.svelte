<script lang="ts">
  import type { ComponentProps, Snippet } from "svelte";
  import Button from "./Button.svelte";
  import { ChevronDownIcon } from "@lucide/svelte";
  import { twMerge } from "tailwind-merge";
  import InputField from "./input/InputField.svelte";

  type PickedButtonProps = Pick<
    ComponentProps<typeof Button>,
    "variant" | "size" | "class"
  >;

  interface Props extends PickedButtonProps {
    children: Snippet;
    icon: Snippet;
    name: string;
  }

  const { children, variant, icon, size, name }: Partial<Props> = $props();
</script>

<InputField id="lol">
  {#snippet actions()}
    <span class="absolute right-0 inset-y-0 flex items-center">
      <Button icon variant="secondary">
        <ChevronDownIcon size={16} />
      </Button>
    </span>
  {/snippet}
</InputField>
