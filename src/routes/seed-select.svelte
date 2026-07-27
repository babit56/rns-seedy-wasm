<script lang="ts">
	import { urlSeed, type Settings, copySettings } from '$lib/util';
	import type { SeedData } from '$lib/item-map';
	import { Unlocks, StartingArea, int_to_area, predict_seed, new_seed_data } from 'rnssp-wasm';

	type Props = {
		seed_data: SeedData[];
		playerCount: number;
		settings: Settings;
		lastSearchSettings: Settings;
		loading: boolean;
		possible_seeds: SeedData[];
		searched: boolean;
	};

	let {
		seed_data,
		playerCount = $bindable(),
		settings = $bindable(),
		lastSearchSettings = $bindable(),
		loading = $bindable(true),
		possible_seeds = $bindable([]),
		searched = $bindable(false)
	}: Props = $props();

	function get_seed_data() {
		// console.log(seed, 4, difficulty > 1, starting_area, unlocks)
		// Let frontend worry about player counts below 4
		possible_seeds = [predict_seed(seed, 4, settings.difficulty > 1, settings.starting_area, settings.unlocks)];
		searched = true;
		copySettings(lastSearchSettings, settings);
	}

	function reset_seed_data() {
		possible_seeds = [];
		searched = false;
		copySettings(lastSearchSettings, settings);
	}

	let seed = $state<number>(urlSeed ?? 0);

	// Auto search if given a seed
	let firstLoadSearch = urlSeed !== null;
	$effect(() => {
		if (loading === false && firstLoadSearch === true) {
			firstLoadSearch = false;
			get_seed_data();
		}
	});
</script>

<h2 class="page-title">Direct Seed Finder</h2>
<div class="columns">
	<section class="prose">
		<h3>About</h3>
		<p>
			This tool lets you display <strong>a given seed</strong> based on the given settings.
			Any seed can be input, but behavior might be funky for large seeds.
			The seed in the results will always match <strong>a</strong> correct seed ingame
		</p>
		<h3>Usage</h3>
		<p>Type in the seed.</p>
	</section>
	<section class="input-section">
		<fieldset class="input-area">
			<legend>Literally just the seed</legend>
			<label>Seed <input type="number" min="0" name="seed" bind:value={seed} /></label>
		</fieldset>
		<div class="button-group">
			<button class="action-button" disabled={loading} onclick={get_seed_data}>Search</button>
			<button class="action-button outlined-button" disabled={loading} onclick={reset_seed_data}
				>Reset</button
			>
		</div>
	</section>
</div>

<style>
	label {
		display: flex;
		gap: 1ch;
		align-items: center;
	}
	input {
		width: 100%;
	}
</style>
