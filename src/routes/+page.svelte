<script lang="ts">
	import seed_data from '$lib/seed-data.json';
	import { name_to_id, itemList, gem_to_id } from '$lib/item-map';
	import { Seed } from '$lib/seed';
	import Combobox from './combobox.svelte';

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

	const items = itemList.map((item) => ({ value: item, label: item }));
	const gems = ['Opal', 'Sapphire', 'Ruby', 'Garnet', 'Emerald'].map((item) => ({
		value: item,
		label: item
	}));
</script>

<h2 class="page-title">Seed-In-Progress finder</h2>
<div class="columns">
	<section class="prose">
		<h3>What it Does</h3>
		<p>
			This tool allows you to find your <strong>current seed</strong> in a Hard or Lunar difficulty run.
			Loot and shops are generated separately, so you need one chest of loot to figure out chests and
			one shop to figure out shops
		</p>
		<h3>How to Use</h3>
		<p>
			Enter first chest's items and press Search to find all future items. You can optionally add
			the first shop's gems to find out what the rest of the run's gems will be.
		</p>
		<p>
			The first chest is read counterclockwise starting from the top left. You can also press the
			View Inventory button and then read the list left to right. Gems are read left to right.
		</p>
	</section>
	<section>
		<fieldset class="input-area">
			<legend>First chest loot</legend>
			<label>
				Item 1
				<Combobox type="single" {items} bind:value={item_1} />
			</label>
			<label>
				Item 2
				<Combobox type="single" {items} bind:value={item_2} />
			</label>
			<label>
				Item 3
				<Combobox type="single" {items} bind:value={item_3} />
			</label>
			<label>
				Item 4
				<Combobox type="single" {items} bind:value={item_4} />
			</label>
			<label>
				Item 5
				<Combobox type="single" {items} bind:value={item_5} />
			</label>
			<datalist id="items">
				{#each itemList as item}
					<option value={item}></option>
				{/each}
			</datalist>
		</fieldset>
		<fieldset class="input-area">
			<legend>Shop gems <em>(optional)</em></legend>
			<label>
				Primary Gem
				<Combobox type="single" items={gems} bind:value={gem_1} />
			</label>
			<label>
				Secondary Gem
				<Combobox type="single" items={gems} bind:value={gem_2} />
			</label>
			<label>
				Special Gem
				<Combobox type="single" items={gems} bind:value={gem_3} />
			</label>
			<label>
				Defensive Gem
				<Combobox type="single" items={gems} bind:value={gem_4} />
			</label>
		</fieldset>
		<div class="button-group">
			<button class="action-button" onclick={get_seed_data}>Search</button>
			<button class="action-button outlined-button" onclick={reset_seed_data}>Reset</button>
		</div>
	</section>
</div>

<h2>Results</h2>
{#if found_seeds.length > 0}
	<div class="seed-list">
		{#each found_seeds as seed}
			<h3>Seed {seed.id}</h3>
			<p>Areas (4 and 5 are unused)</p>
			<p>
				{#each seed.areas as area_name, index}
					<span class:muted-text={index >= 3}>{area_name}</span>{#if index < 4},&nbsp;{/if}
				{/each}
			</p>
			<p>Chests</p>
			<ul>
				{#each [1, 2, 3, 4, 5, 6] as chest_number, chest_index}
					<li>
						Chest {chest_number}:
						{#each seed.chest(chest_index) as item, index}
							<span>{item}</span>{#if index < 4},&nbsp;{/if}
						{/each}
					</li>
				{/each}
			</ul>
			<p>Shops</p>
			<ul>
				<ul>
					{#each [1, 2, 3, 4] as shop_number, shop_index}
						<li>
							Shop {shop_number}:
							<ul>
								<li>
									Potions: {#each seed.shop(shop_index)?.potions as gem, index}
										<span>{gem.name} ({gem.price})</span>{#if index < 3},&nbsp;{/if}
									{/each}
								</li>
								<li>
									Gems: {#each seed.shop(shop_index)?.gems as gem, index}
										<span>{gem.name} ({gem.price})</span>{#if index < 3},&nbsp;{/if}
									{/each}
								</li>
							</ul>
						</li>
					{/each}
				</ul>
			</ul>
		{/each}
	</div>
{:else if !searched}
	<p><em>Search for your seed!</em></p>
{:else}
	<p>No seed found...</p>
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
		column-gap: 1rem;
	}

	.input-area {
		padding: 0.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 0.75rem;

		label {
			display: flex;
			gap: 1ch;
			align-items: center;
		}
	}

	.outlined-button {
		background-color: unset;
		border: var(--border-size-2) solid var(--surface-4);
		transition: border-color 100ms;
		&:hover {
			border-color: color-mix(in lch, var(--surface-4), rgb(from var(--surface-1) r g b) 20%);
		}
		&:active {
			border-color: color-mix(in lch, var(--surface-4), rgb(from var(--surface-1) r g b) 30%);
		}
	}

	.seed-list {
		padding: 1rem;
		background-color: var(--surface-1);
		border: var(--border-size-1) solid var(--surface-4);
		border-radius: var(--radius-2);
	}

	.button-group {
		display: flex;
		gap: 1rem;
	}
</style>
