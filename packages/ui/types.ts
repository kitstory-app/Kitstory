import type { Snippet } from "svelte"

export type WithChildrenSnippet<Props extends object> = { children?: Snippet } & Props
