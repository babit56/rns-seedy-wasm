<script lang="ts">
	import PlayerCount from './player-count.svelte';
	import SeedDisplay from './seed-display.svelte';
	import { urlSeed, urlTab } from '$lib/util';
	import type { SeedData } from '$lib/item-map';
	import { onMount } from 'svelte';
	import SeedInProgress from './seed-in-progress.svelte';
	import SeedSelect from './seed-select.svelte';
	import { Tabs } from 'bits-ui';
	import SeedSearch from './seed-search.svelte';
	import Pagination from './pagination.svelte';
	import Switch from './switch.svelte';
	import WizardSpinner from './wizard-spinner.svelte';

	onMount(async () => {
		seed_data = await (await fetch('data/seed-data.json')).json(); // Static assets makes the IDE happy
		loading = false;
	});

	const tabList = ['progress', 'select', 'search'] as const;
	type AppTab = (typeof tabList)[number];

	const seedTab: AppTab | null = urlSeed !== null ? 'select' : null;
	const urlTabName: AppTab | null = urlTab
		? tabList.includes(urlTab as AppTab)
			? (urlTab as AppTab)
			: null
		: null;
	const initialTab: AppTab = seedTab ?? urlTabName ?? 'progress';

	let currentTab = $state(initialTab);

	// Cannot be a rune for performance reasons...
	// svelte-ignore non_reactive_update
	let seed_data: SeedData[] = [];

	let loading = $state(true);
	let searched = $state(false);
	let playerCount = $state(4);

	let seedPage = $state(1);
	let perPage = 10;

	let found_seeds = $state<SeedData[]>([]);

	// Reset page on seed change
	$effect(() => {
		found_seeds.length;
		seedPage = 1;
	});

	// Sync url search parameters
	$effect(() => {
		const tabbedUrl = new URL(window.location.href);
		if (currentTab === 'progress') {
			tabbedUrl.searchParams.delete('tab');
		} else {
			tabbedUrl.searchParams.set('tab', currentTab);
		}

		window.history.pushState({}, '', tabbedUrl);
	});

	// Sync seed if a single result is returned
	$effect(() => {
		if (found_seeds.length !== 1) {
			const noSeedUrl = new URL(window.location.href);
			noSeedUrl.searchParams.delete('seed');

			window.history.pushState({}, '', noSeedUrl);
		} else {
			const seedUrl = new URL(window.location.href);
			seedUrl.searchParams.set('seed', String(found_seeds[0][0]));

			window.history.pushState({}, '', seedUrl);
		}
	});

	let showPagination = $derived(found_seeds.length > perPage);
	let seedWindow = $derived(found_seeds.slice(perPage * (seedPage - 1), perPage * seedPage));

	let compactSeeds = $state(false);

	let showPercentage = $derived(found_seeds.length > 50);
</script>

<Tabs.Root bind:value={currentTab}>
	<Tabs.List class="tab-list">
		<Tabs.Trigger value="progress" class="outlined-button tab-button">
			Seed-In-Progress Finder
		</Tabs.Trigger>
		<Tabs.Trigger value="select" class="outlined-button tab-button">
			Direct Seed Finder
		</Tabs.Trigger>
		<Tabs.Trigger value="search" class="outlined-button tab-button">End Seed Searcher</Tabs.Trigger>
	</Tabs.List>
	<Tabs.Content value="progress">
		<SeedInProgress {seed_data} bind:possible_seeds={found_seeds} bind:searched bind:loading />
	</Tabs.Content>
	<Tabs.Content value="select">
		<SeedSelect {seed_data} bind:possible_seeds={found_seeds} bind:searched bind:loading />
	</Tabs.Content>
	<Tabs.Content value="search">
		<SeedSearch {seed_data} bind:possible_seeds={found_seeds} bind:searched bind:loading />
	</Tabs.Content>
</Tabs.Root>
<div class="results-header">
	<h2>
		Results {#if searched}<span class="results-count"
				>({found_seeds.length}{#if showPercentage}&nbsp;:&nbsp;{(
						(found_seeds.length * 100) /
						seed_data.length
					).toFixed(2)}%{/if})</span
			>{/if}
	</h2>
	<div class="results-options">
		<div class="results-options-bg"></div>
		<Switch labelText="Compact" bind:checked={compactSeeds} />
		<PlayerCount bind:value={playerCount} />
	</div>
</div>
{#if found_seeds.length > 0}
	{#if showPagination}
		<Pagination count={found_seeds.length} {perPage} bind:page={seedPage}></Pagination>
	{/if}
	<div class="seed-list">
		{#each seedWindow as seedData (seedData[0])}
			<SeedDisplay {seedData} {playerCount} compact={compactSeeds} />
		{/each}
	</div>
	{#if showPagination}
		<Pagination count={found_seeds.length} {perPage} bind:page={seedPage}></Pagination>
	{/if}
{:else}
	<div class="search-loader">
		{#if searched}
			<p>No seed found...</p>
		{:else}
			<WizardSpinner />
			<p>Search for a seed above!</p>
		{/if}
	</div>
{/if}

<style>
	.seed-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-block: var(--size-4);
	}

	.results-header {
		margin-top: 2.5rem;
		display: flex;
		flex-wrap: wrap;
		row-gap: var(--size-1);
		margin-bottom: var(--size-2);
		justify-content: space-between;
		align-items: center;
		overflow-x: clip;

		h2 {
			display: flex;
			align-items: center;
			gap: 0.25em;
		}
	}

	.results-count {
		font-weight: var(--font-weight-6);
		font-size: var(--font-size-5);
		color: var(--text-2);
		display: inline-block;
	}

	.results-options {
		display: flex;
		gap: var(--size-3);
		position: relative;

		--c: color-mix(in lch, var(--surface-3), transparent 20%);

		.results-options-bg::before {
			content: '';
			position: absolute;
			--overhang-x: 80px;
			--overhang-y: 4px;
			width: calc(100% + var(--overhang-x));
			left: calc(var(--overhang-x) * -1);
			top: calc(var(--overhang-y) * -1);
			height: calc(100% + var(--overhang-y) * 2);
			background-image: linear-gradient(
				-45deg,
				transparent,
				transparent 18.3707517568%,
				var(--c) 0,
				var(--c) 31.6292482432%,
				transparent 0,
				transparent 68.3707517568%,
				var(--c) 0,
				var(--c) 81.6292482432%,
				transparent 0,
				transparent
			);
			background-repeat: repeat;
			background-size: 0.75rem 0.75rem;
			mask-image: linear-gradient(150deg, transparent, transparent 5%, black 30%, black);
			z-index: -1;
		}
	}

	:global(.tab-list) {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: var(--size-3);
		margin-bottom: var(--size-4);
	}
	:global(.tab-button) {
		position: relative;
		overflow: hidden;
		font-weight: var(--font-weight-6);
		font-size: var(--font-size-2);
		color: var(--text-2);
		transition: color 100ms linear;

		&[data-state='active'] {
			color: var(--text-1);
		}

		--c: color-mix(in lch, var(--surface-3), transparent 20%);

		&::before {
			content: '';
			z-index: -1;
			position: absolute;
			inset: 0;
			background-image: linear-gradient(
				-45deg,
				transparent,
				transparent 18.3707517568%,
				var(--c) 0,
				var(--c) 31.6292482432%,
				transparent 0,
				transparent 68.3707517568%,
				var(--c) 0,
				var(--c) 81.6292482432%,
				transparent 0,
				transparent
			);
			background-repeat: repeat;
			background-size: 0.75rem 0.75rem;
			mask-image: linear-gradient(
				90deg,
				black,
				black calc(5% * var(--tab-bg-offset)),
				transparent calc(80% * var(--tab-bg-offset)),
				transparent
			);

			transition: --tab-bg-offset 300ms cubic-bezier(0.16, 1, 0.3, 1);

			--tab-bg-offset: 0;
		}
		&[data-state='active']::before {
			--tab-bg-offset: 1;
		}
	}

	@property --tab-bg-offset {
		syntax: '<number>';
		initial-value: 0;
		inherits: false;
	}

	.search-loader {
		display: flex;
		flex-direction: column;
		gap: var(--size-2);
		align-items: center;
		padding: var(--size-4);
		color: var(--text-2);
		margin-top: var(--size-4);
		background-color: var(--surface-1);
		border: var(--border-size-1) solid var(--surface-2);
		border-radius: var(--radius-2);
	}
</style>
