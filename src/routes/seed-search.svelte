<script lang="ts">
	import { gem_to_id, itemList, name_to_id, type SeedData } from '$lib/item-map';
	import Combobox from './combobox.svelte';
	import { type Settings, copySettings } from '$lib/util';

	type Props = {
		seed_data: SeedData[];
		loading: boolean;
		possible_seeds: SeedData[];
		searching?: boolean; // Currently not used because searches are fast!
		searched: boolean;
		settings: Settings;
		lastSearchSettings: Settings;
	};

	let {
		seed_data,
		loading = $bindable(true),
		possible_seeds = $bindable([]),
		searching = $bindable(false),
		searched = $bindable(false),
		settings = $bindable(),
		lastSearchSettings = $bindable(),
	}: Props = $props();

	let item_1 = $state('');
	let item_2 = $state('');
	let item_3 = $state('');
	let item_4 = $state('');
	let item_5 = $state('');
	let item_6 = $state('');
	let item_1_id = $derived(name_to_id(item_1));
	let item_2_id = $derived(name_to_id(item_2));
	let item_3_id = $derived(name_to_id(item_3));
	let item_4_id = $derived(name_to_id(item_4));
	let item_5_id = $derived(name_to_id(item_5));
	let item_6_id = $derived(name_to_id(item_6));

	let area_1 = $state('');
	let area_2 = $state('');
	let area_3 = $state('');
	let area_1_id = $derived(Number(area_1));
	let area_2_id = $derived(Number(area_2));
	let area_3_id = $derived(Number(area_3));

	let gem_1 = $state('');
	let gem_2 = $state('');
	let gem_3 = $state('');
	let gem_4 = $state('');
	let gem_1_id = $derived(gem_to_id(gem_1, 0));
	let gem_2_id = $derived(gem_to_id(gem_2, 1));
	let gem_3_id = $derived(gem_to_id(gem_3, 2));
	let gem_4_id = $derived(gem_to_id(gem_4, 3));

	function get_seed_data() {
		if (searching) return;

		possible_seeds = [];
		searching = true;
		searched = false;

		// Rough match for possibly matching seeds
		const matched_seeds = seed_data.filter((seed) => {
			const itemMatch = [item_1_id, item_2_id, item_3_id, item_4_id, item_5_id, item_6_id].every(
				(id, index) =>
					id === 0 || seed.slice(index * 5 + 6, index * 5 + 5 + 6).includes(id) // Blank = Match any
			);
			if (!itemMatch) return false;
			const areasMatch = [area_1_id, area_2_id, area_3_id].every(
				(id, index) => id === 0 || seed.at(index + 2) === id // Blank = Match any
			);
			if (!areasMatch) return false;

			// Limit by gems on the smaller set for performance
			const gemsMatch = [gem_1_id, gem_2_id, gem_3_id, gem_4_id].every(
				(id, index) =>
					id === undefined ||
					[0, 1, 2, 3].some((shop) => seed.at(shop * 14 + index * 2 + 42) === id) // Blank = Match any
			);
			if (!gemsMatch) return false;

			return true;
		});

		possible_seeds = matched_seeds;
		searching = false;
		searched = true;
		copySettings(lastSearchSettings, settings);
	}

	function reset_seed_data() {
		item_1 = '';
		item_2 = '';
		item_3 = '';
		item_4 = '';
		item_5 = '';
		item_6 = '';

		area_1 = '';
		area_2 = '';
		area_3 = '';

		gem_1 = '';
		gem_2 = '';
		gem_3 = '';
		gem_4 = '';

		possible_seeds = [];
		searched = false;
		copySettings(lastSearchSettings, settings);
	}

	const items = itemList.map((item) => ({ value: item, label: item }));
	const areas = [
		// { value: 0, label: "Kingdom Outskirts" },
		{ value: "1", label: "Scholar's Nest (Crows)" },
		{ value: "2", label: "King's Aresenal (Wolves)" },
		{ value: "3", label: "Red Darkhouse (Dragons)" },
		{ value: "4", label: "Churchmouse Streets (Mice)" },
		{ value: "5", label: "Emerald Lakeside (Frogs)" },
		// { value: 6, label: "Moonlit Prescipice" },
		// { value: 7, label: "Crack in the Geode" },
		{ value: "8", label: "Subterra Sanctum" },
		{ value: "9", label: "Darkhouse Depths" },
		{ value: "10", label: "Atelier Aurum" },
		// { value: 11, label: "Looping Hallway" },
	];
	const gems = ['Opal', 'Sapphire', 'Ruby', 'Garnet', 'Emerald'].map((item) => ({
		value: item,
		label: item
	}));
</script>

<h2 class="page-title">End Seed Searcher</h2>
<div class="columns">
	<section class="prose">
		<h3>About</h3>
		<p>
			This tool allows you to find your <strong>a seed given items and areas</strong> in a Hard or Lunar
			difficulty run, for if you have the end screen but not a specific chest.
		</p>
		<p>
			This currently does not account for skipped items, so find someone who got an item at every
			chest. Also, this does not account for 1-3 player games so the results may show seeds that are
			not possible.
		</p>
		<h3>Usage</h3>
		<p>
			To find the seed behind someone's run, enter every item that someone has picked in order that
			they picked them. You can optionally add the areas to filter further if you hit multiple
			possible seeds or by gem availability in shops.
		</p>
		<p>
			To find seeds with specific parameters, enter them. E.g. you can find seeds with a Mountain
			Staff in the first chest and a Ruby Special available somewhere in the run.
		</p>
	</section>
	<section class="input-section">
		<fieldset class="input-area">
			<legend>Items</legend>
			<div class="combobox-aligned-input">
				<p class="combobox-label">Item 1</p>
				<Combobox type="single" {items} bind:value={item_1} disabled={searching} />
				<p class="combobox-label">Item 2</p>
				<Combobox type="single" {items} bind:value={item_2} disabled={searching} />
				<p class="combobox-label">Item 3</p>
				<Combobox type="single" {items} bind:value={item_3} disabled={searching} />
				<p class="combobox-label">Item 4</p>
				<Combobox type="single" {items} bind:value={item_4} disabled={searching} />
				<p class="combobox-label">Item 5</p>
				<Combobox type="single" {items} bind:value={item_5} disabled={searching} />
				<p class="combobox-label">Item 6</p>
				<Combobox type="single" {items} bind:value={item_6} disabled={searching} />
			</div>
		</fieldset>
		<fieldset class="input-area">
			<legend>Areas</legend>
			<div class="combobox-aligned-input">
				<p class="combobox-label">Area 1</p>
				<Combobox type="single" items={areas} bind:value={area_1} disabled={searching} />
				<p class="combobox-label">Area 2</p>
				<Combobox type="single" items={areas} bind:value={area_2} disabled={searching} />
				<p class="combobox-label">Area 3</p>
				<Combobox type="single" items={areas} bind:value={area_3} disabled={searching} />
			</div>
		</fieldset>
		<fieldset class="input-area">
			<legend>Shop gems</legend>
			<div class="combobox-aligned-input">
				<p class="combobox-label">Primary Gem</p>
				<Combobox type="single" items={gems} bind:value={gem_1} disabled={searching} />
				<p class="combobox-label">Secondary Gem</p>
				<Combobox type="single" items={gems} bind:value={gem_2} disabled={searching} />
				<p class="combobox-label">Special Gem</p>
				<Combobox type="single" items={gems} bind:value={gem_3} disabled={searching} />
				<p class="combobox-label">Defensive Gem</p>
				<Combobox type="single" items={gems} bind:value={gem_4} disabled={searching} />
			</div>
		</fieldset>
		<div class="button-group">
			<button class="action-button" disabled={loading || searching} onclick={get_seed_data}
				>Search</button
			>
			<button
				class="action-button outlined-button"
				disabled={loading || searching}
				onclick={reset_seed_data}>Reset</button
			>
		</div>
	</section>
</div>
