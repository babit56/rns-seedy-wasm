<script lang="ts">
	import PlayerCount from './player-count.svelte';
	import SeedDisplay from './seed-display.svelte';
	import { Seed } from '$lib/seed';
	import type { SeedData } from '$lib/item-map';
	import { onMount } from 'svelte';
	import SeedInProgress from './seed-in-progress.svelte';
	import SeedSelect from './seed-select.svelte';
	import { Tabs } from 'bits-ui';
	import SeedSearch from './seed-search.svelte';
	import Pagination from './pagination.svelte';

	onMount(async () => {
		seed_data = await (await fetch('data/seed-data.json')).json(); // Static assets makes the IDE happy
		loading = false;
	});

	// Cannot be a rune for performance reasons...
	// svelte-ignore non_reactive_update
	let seed_data: SeedData[] = [];

	let loading = $state(true);
	let searched = $state(false);
	let playerCount = $state(4);

	let seedPage = $state(1);
	let perPage = 10;

	let found_seeds = $state<Seed[]>([]);

	// Reset page on seed change
	$effect(() => {
		found_seeds.length;
		seedPage = 1;
	});

	let showPagination = $derived(found_seeds.length > perPage);
	let seedWindow = $derived(found_seeds.slice(perPage * (seedPage - 1), perPage * seedPage));
</script>

<Tabs.Root value="progress">
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
<div class="result-header">
	<h2>Results</h2>
	<PlayerCount bind:value={playerCount} />
</div>
{#if found_seeds.length > 0}
	{#if showPagination}
		<Pagination count={found_seeds.length} {perPage} bind:page={seedPage}></Pagination>
	{/if}
	<div class="seed-list">
		{#each seedWindow as seed}
			<SeedDisplay {seed} {playerCount} />
		{/each}
	</div>
	{#if showPagination}
		<Pagination count={found_seeds.length} {perPage} bind:page={seedPage}></Pagination>
	{/if}
{:else if searched}
	<p class="no-seed">No seed found...</p>
{:else}
	<p>Search for a seed first!</p>
{/if}

<style>
	.seed-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-block: var(--size-4);
	}
	.result-header {
		margin-top: 2.5rem;
		display: flex;
		flex-wrap: wrap;
		row-gap: var(--size-1);
		margin-bottom: var(--size-2);
		justify-content: space-between;
		align-items: center;
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

	.no-seed {
		color: var(--text-2);
	}
</style>
