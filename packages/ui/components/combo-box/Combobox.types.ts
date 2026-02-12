import type Button from "../Button.svelte"
import type { ComponentProps } from "svelte"

export type PluckedButtonProps = Pick<ComponentProps<typeof Button>, "variant" | "size" | "class">