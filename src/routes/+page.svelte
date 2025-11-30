<script lang="ts">
	import seed_data from '$lib/seed-data.json'; // BIG BOYE
	import { name_to_id, itemList, gem_to_id } from '$lib/item-map';
	import { Seed } from '$lib/seed';
	import Combobox from './combobox.svelte';
	import PlayerCount from './player-count.svelte';
	import SeedDisplay from './seed-display.svelte';
	import { browser } from '$app/environment';

	let item_1 = $state('');
	let item_2 = $state('');
	let item_3 = $state('');
	let item_4 = $state('');
	let item_5 = $state('');
	let item_1_id = $derived(name_to_id(item_1));
	let item_2_id = $derived(name_to_id(item_2));
	let item_3_id = $derived(name_to_id(item_3));
	let item_4_id = $derived(name_to_id(item_4));
	let item_5_id = $derived(name_to_id(item_5));

	let gem_1 = $state('');
	let gem_2 = $state('');
	let gem_3 = $state('');
	let gem_4 = $state('');
	let gem_1_id = $derived(gem_to_id(gem_1, 0));
	let gem_2_id = $derived(gem_to_id(gem_2, 1));
	let gem_3_id = $derived(gem_to_id(gem_3, 2));
	let gem_4_id = $derived(gem_to_id(gem_4, 3));

	function loadExampleSeed() {
		found_seeds = [new Seed(seed_data[0])];
		searched = true;
	}

	function get_seed_data() {
		const matched_seeds = seed_data.filter((seed: Array<string | number>) => {
			// Match first 5 items
			const all_items_match =
				seed[6] === item_1_id &&
				seed[7] === item_2_id &&
				seed[8] === item_3_id &&
				seed[9] === item_4_id &&
				seed[10] === item_5_id;
			// Items and prices are adjacent so index are every 2
			// Optionally can be ignored
			const all_gems_match =
				(gem_1 === '' || seed[42] === gem_1_id) &&
				(gem_2 === '' || seed[44] === gem_2_id) &&
				(gem_3 === '' || seed[46] === gem_3_id) &&
				(gem_4 === '' || seed[48] === gem_4_id);
			if (all_items_match && all_gems_match) {
				return true;
			}
			return false;
		});

		found_seeds = [...matched_seeds.map((s: Array<string | number>) => new Seed(s))];
		searched = true;
	}

	function reset_seed_data() {
		item_1 = '';
		item_2 = '';
		item_3 = '';
		item_4 = '';
		item_5 = '';
		gem_1 = '';
		gem_2 = '';
		gem_3 = '';
		gem_4 = '';

		found_seeds = [];
		searched = false;
	}

	let searched = $state(false);
	let found_seeds = $state<Seed[]>([]);

	let playerCount = $state(4);

	const items = itemList.map((item) => ({ value: item, label: item }));
	const gems = ['Opal', 'Sapphire', 'Ruby', 'Garnet', 'Emerald'].map((item) => ({
		value: item,
		label: item
	}));
</script>

<h2 class="page-title">Seed-In-Progress finder</h2>
<div class="columns">
	<section class="prose">
		<h3>About</h3>
		<p>
			This tool allows you to find your <strong>current seed</strong> in a Hard or Lunar difficulty run.
			Loot and shops are generated separately, so you need one chest of loot to figure out chests and
			one shop to figure out shops
		</p>
		<h3>Usage</h3>
		<p>
			Enter first chest's items and press Search to find all future items. You can optionally add
			the first shop's gems to find out what the rest of the run's gems will be.
		</p>
		<p>
			The first chest is read counterclockwise starting from the top left. You can also press the
			View Inventory button and then read the list left to right. Gems are read left to right.
		</p>
	</section>
	<section class="input-section">
		<fieldset class="input-area">
			<legend>First chest loot</legend>
			<div class="combobox-aligned-input">
				<p class="combobox-label">Item 1</p>
				<Combobox type="single" {items} bind:value={item_1} />
				<p class="combobox-label">Item 2</p>
				<Combobox type="single" {items} bind:value={item_2} />
				<p class="combobox-label">Item 3</p>
				<Combobox type="single" {items} bind:value={item_3} />
				<p class="combobox-label">Item 4</p>
				<Combobox type="single" {items} bind:value={item_4} />
				<p class="combobox-label">Item 5</p>
				<Combobox type="single" {items} bind:value={item_5} />
			</div>
		</fieldset>
		<fieldset class="input-area">
			<legend>Shop gems <em>(optional)</em></legend>
			<div class="combobox-aligned-input">
				<p class="combobox-label">Primary Gem</p>
				<Combobox type="single" items={gems} bind:value={gem_1} />
				<p class="combobox-label">Secondary Gem</p>
				<Combobox type="single" items={gems} bind:value={gem_2} />
				<p class="combobox-label">Special Gem</p>
				<Combobox type="single" items={gems} bind:value={gem_3} />
				<p class="combobox-label">Defensive Gem</p>
				<Combobox type="single" items={gems} bind:value={gem_4} />
			</div>
		</fieldset>
		<div class="button-group">
			<button class="action-button" disabled={!browser} onclick={get_seed_data}>Search</button>
			<button class="action-button outlined-button" disabled={!browser} onclick={reset_seed_data}
				>Reset</button
			>
		</div>
	</section>
</div>
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
	.page-title {
		text-align: center;
		max-inline-size: unset;
		padding-bottom: 1em;
	}

	.columns {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
		column-gap: 2rem;
	}

	.input-section {
		position: relative;
	}

	.input-area {
		padding: 0.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.combobox-label {
		font-size: var(--font-size-1);
	}

	.combobox-aligned-input {
		display: grid;
		grid-template-columns: auto 1fr;
		align-items: center;
		gap: var(--size-2) 1ch;
	}

	.inline-button {
		width: unset;
	}

	.seed-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.button-group {
		display: flex;
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
