<script lang="ts">
	import IconGearRegular from 'phosphor-icons-svelte/IconGearRegular.svelte';
	import IconSealCheckFill from 'phosphor-icons-svelte/IconSealCheckFill.svelte';
	import IconSealWarningFill from 'phosphor-icons-svelte/IconSealWarningFill.svelte';
	import { Dialog } from 'bits-ui';
	import { urlSeed, difficulty_to_icon, type Settings, isSettingsEqual,
		copySettings, settingsHavePrecalc, getPrecalcFile } from '$lib/util';
	import type { SeedData, AreaName } from '$lib/item-map';
	import { starting_area_to_icon, itemsets, itemsetColorMap, itemset_index_to_icon } from '$lib/item-map';
	import { Unlocks, StartingArea, new_seed_data, int_to_area, predict_seed } from 'rnssp-wasm';
	import Difficulty from './difficulty.svelte';

	type Props = {
		seed_data: SeedData[];
		settings: Settings;
		pendingSettings: Settings;
		loading: boolean;
	};

	let {
		seed_data = $bindable(),
		settings = $bindable(),
		pendingSettings = $bindable(),
		loading = $bindable(true),
	}: Props = $props();

	const unlockableSets = itemsets.slice(15);

	function change_start(area_index: number, event) {
		const old = event.target.parentElement.querySelectorAll(".selection-options")[pendingSettings.starting_area];
		old.classList.toggle('chosen');
		pendingSettings.starting_area = int_to_area(area_index);
		event.target.classList.toggle('chosen');
	}

	function change_diff(new_diff: number, event) {
		const old = event.target.parentElement.querySelectorAll(".selection-options")[pendingSettings.difficulty];
		old.classList.toggle('chosen');
		pendingSettings.difficulty = new_diff
		event.target.classList.toggle('chosen');
	}

	async function change_seed_data() {
		loading = true;
		copySettings(settings, pendingSettings);
		if (settingsHavePrecalc(settings)) {
			const resource = getPrecalcFile(settings);
			console.log("Loading seed data from:", resource);
			seed_data = await (await fetch(resource)).json();
		} else {
			console.log("Calculating new seed data");
			const start = Date.now();
			seed_data = seed_data.map((s) => predict_seed(s[0], 4, settings.difficulty, settings.starting_area, settings.unlocks));

			// Other option, runtime seems to be equal. Further benchmarking probably needed
			// const unique_seeds = seed_data.map((s) => s[0]);
			// console.log(new_seed_data(unique_seeds, 4, settings.difficulty, settings.starting_area, settings.unlocks));

			const ms = Date.now() - start;
			console.log(`Changed seed data in ${ms / 1000} seconds`);
		}
		loading = false;
	}
</script>

{#snippet inlineIcon(src: string, alt: string | null = null)}
	<img class="inline-icon" {src} {alt} />
{/snippet}

<Dialog.Root>
	<Dialog.Trigger class="settings-btn">
			<IconGearRegular />
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Overlay class="settings-overlay" />
		<Dialog.Content class="settings-content">
			<Dialog.Title class="settings-title">
				Settings
			</Dialog.Title>
			<div class="columns settings-body">
				<section>
					<Dialog.Description>
						<p>
						Here you can change settings used by the rest of the tools.
						</p>

						<ul>
							<li>Starting areas change areas, shops, and encounters</li>
							<li>Hard/Lunar spawns a Regeneration potion in the first shop and therefore affects that shop</li>
							<li>Finally, item set unlocks change loot generation</li>
							<li>Seed data must be loaded for use in searchers. Finder does not need loading</li>
						</ul>
					</Dialog.Description>
				</section>
				<section class="input-section">
					<fieldset class="area-selection">
						<legend>Starting area{@render inlineIcon("images/starting_area_indicator.png")}</legend>
						{#each {length: 12} as _, i}
							<img
								class="selection-options"
								class:chosen={pendingSettings.starting_area === i}
								src={"images/starting_area/" + starting_area_to_icon(i)}
								alt={`Starting area ${i}`}
								onclick={(e) => change_start(i, e)}
							/>
						{/each}
					</fieldset>
				</section>
				<section class="input-section">
					<fieldset class="difficulty-selection">
						<legend>Difficulty{@render inlineIcon("images/difficulty_marker.png")}</legend>
						{#each {length: 4} as _, i}
							<img
								class="selection-options"
								class:chosen={pendingSettings.difficulty === i}
								src={"images/difficulty/" + difficulty_to_icon(i)}
								alt={`Difficulty ${i}`}
								onclick={(e) => change_diff(i, e)}
							/>
						{/each}
					</fieldset>
					<fieldset class="seeddata-loader">
						<legend>Load new seed data</legend>
						<div hidden={!isSettingsEqual(settings, pendingSettings)}>
							<IconSealCheckFill class="seal-check inline-icon" />
							Seed data is up to date
						</div>
						<div hidden={isSettingsEqual(settings, pendingSettings)}>
							<IconSealWarningFill class="seal-warning inline-icon" />
							Seed data needs reload!
						</div>
						<div hidden={!settingsHavePrecalc(pendingSettings)}>
							<IconSealCheckFill class="seal-check inline-icon" />
							Settings have precalculated data
						</div>
						<div
						<div hidden={settingsHavePrecalc(pendingSettings)}>
							<IconSealWarningFill class="seal-warning inline-icon" />
							Seed data needs to be calculated, takes ~15-120 seconds
						</div>
						<button onclick={change_seed_data}>Reload seed data</button>
					</fieldset>
				</section>
				<section class="input-section">
					<fieldset class="unlocks-selection">
						<legend>Unlocks{@render inlineIcon("images/toybox.png")}</legend>
						{#each {length: 20} as _, i}
							<img
								class="selection-options"
								class:toggled={pendingSettings.unlocks.check_index(i)}
								src={"images/loot/" + itemset_index_to_icon(unlockableSets[i], 0)}
								alt={unlockableSets[i]}
								onclick={() => {
									pendingSettings.unlocks.toggle_index(i);
									// Make sure svelte notices that stuff changed
									pendingSettings = { ...pendingSettings };
								}}
								style={`--background-color: ${itemsetColorMap[unlockableSets[i]]}`}
							/>
						{/each}
					</fieldset>
				</section>
			</div>
			<Dialog.Close class="settings-close">
				Close
			</Dialog.Close>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

<style>
	:global(.settings-btn) {
		width: 48px;
		height: 48px;
		// opacity: 0;
	  padding: 0;
	  border: none;
	  border-radius: 50%;
	  display: inline-flex;
	  align-items: center;
	  justify-content: center;
	 //  cursor: pointer;
	 //  transition:
	 //    background-color 0.2s ease,
	 //    transform 0.15s ease,
	 //    box-shadow 0.2s ease;
	}
	:global(.settings-btn svg) {
	  width: 36px;
	  height: 36px;
	  display: block;
	  flex-shrink: 0;
	}
	:global(.settings-title) {
		text-align: center;
		font-size: var(--font-size-4);
		font-weight: var(--font-weight-6);
		margin-bottom: 1rem;
	}
	:global(.settings-overlay) {
		position: fixed;
		inset: 0;
		background-color: var(--surface-overlay);
	}
	:global(.settings-content) {
		position: fixed;
		width: 800px;
		max-width: calc(100vw - 2rem);
		left: 50%;
		top: 50%;
		translate: -50% -50%;
		padding: var(--size-3);

		background-color: var(--surface-2);
		border-radius: var(--size-2);
		box-shadow: var(--shadow-2);
	}
	:global(.settings-close) {
		width: 20%
	}
	.settings-body {
		max-height: 80vh;
		overflow-y: auto;
	}
	
  .selection-options {
  	width: 100%;
  	aspect-ratioo: 1;
  	object-fit: cover;
  	display: block;
  }
  .area-selection {
  	display: grid;
  	grid-template-columns: repeat(4, 1fr);
  	// gap: 0.5rem;
   	// border: 1px solid #ccc;
   	// padding: 1rem;
  }
  .area-selection img.chosen {
    filter:
        drop-shadow(0 0 0 white)
        drop-shadow(2px 0 0 white)
        drop-shadow(-2px 0 0 white)
        drop-shadow(0 2px 0 white)
        drop-shadow(0 -2px 0 white)
        drop-shadow(2px 2px 0 white)
        drop-shadow(-2px -2px 0 white)
        drop-shadow(2px -2px 0 white)
        drop-shadow(-2px 2px 0 white);
  }
  .difficulty-selection {
  	display: grid;
  	grid-template-columns: repeat(4, 1fr);
  	gap: 0.5rem;
   	padding: 1rem;
  }
  .difficulty-selection img.chosen {
  	--outline-color: white;
    filter:
        drop-shadow(0 0 0 var(--outline-color))
        drop-shadow(1px 0 0 var(--outline-color))
        drop-shadow(-1px 0 0 var(--outline-color))
        drop-shadow(0 1px 0 var(--outline-color))
        drop-shadow(0 -1px 0 var(--outline-color))
        drop-shadow(1px 1px 0 var(--outline-color))
        drop-shadow(-1px -1px 0 var(--outline-color))
        drop-shadow(1px -1px 0 var(--outline-color))
        drop-shadow(-1px 1px 0 var(--outline-color));
  }
  .unlocks-selection {
  	display: grid;
  	grid-template-columns: repeat(5, 1fr);
  	gap: 0.5rem;
   	padding: 1rem;
  }
  .unlocks-selection img.toggled {
  	background-color: var(--background-color);
  	background-opacity: 50%;
  }
	:global(.inline-icon) {
		display: inline;
		height: 1.5em;
		vertical-align: bottom;
	}

	:global(.seal-check) {
		height: 1.5em;
		width: 1.5em;
		color: green;
	}

	:global(.seal-warning) {
		height: 1.5em;
		width: 1.5em;
		color: red;
	}

	.seeddata-loader div {
		margin-top: 0.5em;
	}
	.seeddata-loader button {
		margin-top: 0.5em;
	}
</style>
