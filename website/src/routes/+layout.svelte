<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="bg-[#0f0f14] text-white min-h-screen justify-center items-center flex flex-col">
	<div>
		{@render children()}
	</div>
</div>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap');
	* {
		font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
	}
	img {
    	image-rendering: auto;
    	image-rendering: crisp-edges;
    	image-rendering: pixelated;
    	image-rendering: -webkit-optimize-contrast;
	}
</style>