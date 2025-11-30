<script lang="ts">
	import PlayerCount from './player-count.svelte';
	import SeedDisplay from './seed-display.svelte';
	import { Seed } from '$lib/seed';
	import { onMount } from 'svelte';
	import SeedInProgress from './seed-in-progress.svelte';

	onMount(async () => {
		seed_data = (await import('$lib/seed-data.json')).default;
		loading = false;
	});

	// Cannot be a rune for performance reasons...
	let seed_data: any[] = [];

	function loadExampleSeed() {
		found_seeds = [new Seed(seed_data[0])];
		searched = true;
	}

	let loading = $state(true);
	let searched = $state(false);
	let playerCount = $state(4);

	let found_seeds = $state<Seed[]>([]);
</script>

<SeedInProgress {seed_data} bind:possible_seeds={found_seeds} bind:searched bind:loading />
<div class="result-header">
	<h2>Results ({found_seeds.length})</h2>
	<PlayerCount bind:value={playerCount} />
</div>
{#if found_seeds.length > 0}
	<div class="seed-list">
		{#each found_seeds as seed}
			<SeedDisplay {seed} {playerCount} />
		{/each}
	</div>
{:else if !searched}
	<p><em>Search for your seed!</em></p>
{:else}
	<p>No seed found...</p>
	<p>
		Want example? <button class="inline-button" onclick={loadExampleSeed}>Load Seed 0</button>
	</p>
{/if}

<style>
	.inline-button {
		width: unset;
	}

	.seed-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
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
</style>
