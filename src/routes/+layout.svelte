<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import LightDarkSwitcher from './light-dark-switcher.svelte';
	import '../app.css';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<header>
	<h1>RNS Seedy</h1>
	<LightDarkSwitcher />
</header>
<main>
	{@render children()}
</main>

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;

		h1 {
			font-size: 2rem;
		}
	}
</style>
