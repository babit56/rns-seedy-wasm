<script lang="ts">
	import { urlSeed } from '$lib/util';
	import type { SeedData } from '$lib/item-map';
	import { predict_seed } from 'rnssp-wasm';

	type Props = {
		seed_data: SeedData[];
		loading: boolean;
		possible_seeds: SeedData[];
		searched: boolean;
	};

	let {
		seed_data,
		loading = $bindable(true),
		possible_seeds = $bindable([]),
		searched = $bindable(false)
	}: Props = $props();

	function get_seed_data() {
		const foundSeed = seed_data.filter((s) => s[0] === seed).at(0);
		possible_seeds = foundSeed ? [foundSeed] : [];
		searched = true;
	}

	function reset_seed_data() {
		possible_seeds = [];
		searched = false;
		// Change seed data
		const start = Date.now();
		console.log("Changing seed data");
		seed_data = seed_data.map((s) => predict_seed(s[0], 4, false))
		const ms = Date.now() - start;
		console.log(`Changed data in ${ms / 1000} seconds`);
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
			This tool allows you to find your <strong>a given seed</strong> in a Hard or Lunar difficulty run.
		</p>
		<p>
			Only the unique seeds have been recorded for searching, so some seeds in the higher digits
			will not be found.
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
