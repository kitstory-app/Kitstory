<script lang="ts">
	import "../app.css";

	import interFont from "@fontsource/inter/files/inter-latin-400-normal.woff2?url";
	import interFont700 from "@fontsource/inter/files/inter-latin-700-normal.woff2?url";
	import { Navbar } from "$lib/components/navbar";
	import { pwaInfo } from "virtual:pwa-info";
	import Footer from "$lib/components/Footer.svelte";

	const { children } = $props();

	const webManifest = $derived(pwaInfo ? pwaInfo.webManifest.linkTag : "");
</script>

<svelte:head>
	{@html webManifest}
	<!-- Preload fonts -->
	<link
		rel="preload"
		as="font"
		href={interFont}
		crossorigin="anonymous"
		fetchpriority="high"
	/>
	<link
		rel="preload"
		as="font"
		href={interFont700}
		crossorigin="anonymous"
		fetchpriority="high"
	/>
</svelte:head>

<Navbar />
<div id="wrapper" class="flex gap-x-4 mt-(--nav-padding-y)">
	<!-- <aside class="shrink-0">sidebar</aside> -->
	<div
		class="flex-1 dark:bg-neutral-900 bg-white dark:border-neutral-800 selection:bg-amber-300 selection:text-black min-h-[calc(theme(height.dvh)-calc(var(--nav-padding-y))*1)]"
	>
		<main id="skip-content">
			{@render children()}
		</main>
	</div>
</div>
<Footer />
