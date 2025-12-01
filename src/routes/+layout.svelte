<script lang="ts">
	import LightDarkSwitcher from './light-dark-switcher.svelte';
	import Loader from './loader.svelte';
	import { SvelteToast } from '@zerodevx/svelte-toast';
	import '../app.css';
	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" type="image/png" href="favicon-96x96.png" sizes="96x96" />
	<link rel="icon" type="image/svg+xml" href="favicon.svg" />
	<link rel="shortcut icon" href="favicon.ico" />
	<link rel="apple-touch-icon" sizes="180x180" href="apple-touch-icon.png" />
	<meta name="apple-mobile-web-app-title" content="MyWebSite" />
	<link rel="manifest" href="site.webmanifest" />
	<title>RnS Seedy</title>
</svelte:head>

<Loader />
<SvelteToast options={{ intro: { x: 40 } }} />

<header>
	<h1>RnS Seedy</h1>
	<LightDarkSwitcher />
</header>
<main>
	{@render children()}
</main>
<footer>
	<ul class="info">
		<li>Made by FireIsGood</li>
		<li>
			Seed data derived by members of the <a href="https://discord.com/invite/mns" target="_blank"
				>mino_dev Official Discord</a
			>
		</li>
		<li><a href="https://github.com/FireIsGood/rns-seedy">source code</a></li>
	</ul>
	<ul>
		<li>Rabbit &amp; Steel is &copy; mino_dev</li>
		<li>This project is unaffiliated with Rabbit &amp; Steel's creators/distributors</li>
	</ul>
</footer>

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;

		h1 {
			font-size: 2rem;
		}
	}

	footer ul {
		display: flex;
		flex-wrap: wrap;
		gap: var(--size-1) var(--size-2);
		justify-content: center;
		padding: 0;
		list-style: none;
		font-size: var(--font-size-1);
		text-align: center;
		color: var(--text-2);

		& li {
			display: inline-block;
			padding: 0;

			&:not(:last-child)::after {
				content: '\00b7';
				padding-inline-start: var(--size-2);
			}
		}
	}

	@media (width < 800px) {
		:root footer ul {
			justify-content: flex-start;
			text-align: initial;
			padding-bottom: var(--size-2);
			list-style: '\2022  ';
			padding-inline-start: 2ch;

			& li {
				display: list-item;
			}
			& li::after {
				content: unset;
			}
		}
	}
</style>
