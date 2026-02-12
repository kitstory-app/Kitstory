<script lang="ts">
  import type { WithChildrenSnippet } from "../../types";

  interface Props {
    buttons?: boolean
  }

  const { children }: WithChildrenSnippet<Props> = $props();
</script>
