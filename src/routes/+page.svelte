<script lang="ts">
	import seed_data from '$lib/seed-data.json';
	import { name_to_id, itemList, gem_to_id } from '$lib/item-map';
	import { Seed } from '$lib/seed';

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
		const matched_seeds = seed_data.filter((seed) => {
			// DEBUG
			// if (seed[0] === 0) return true;

			// Match first 5 items
			const all_items_match =
				seed[6] === item_1_id &&
				seed[7] === item_2_id &&
				seed[8] === item_3_id &&
				seed[9] === item_4_id &&
				seed[10] === item_5_id;
			// Items and prices are adjacent so index are every 2
			const all_gems_match =
				seed[42] === gem_1_id &&
				seed[44] === gem_2_id &&
				seed[46] === gem_3_id &&
				seed[48] === gem_4_id;
			if (all_items_match && all_gems_match) {
				return true;
			}
			return false;
		});
		console.log(seed_data);
		console.log(matched_seeds);

		found_seeds = [...matched_seeds.map((s) => new Seed(s))];
		searched = true;
	}

	let searched = $state(false);
	let found_seeds = $state<Seed[]>([]);
</script>

<p>Enter first chest items to find your seed.</p>

<section>
	<p>First chest loot</p>
	<fieldset class="input-area">
		<label>
			Item 1
			<input list="items" type="text" bind:value={item_1} />
		</label>
		<label>
			Item 2
			<input list="items" type="text" bind:value={item_2} />
		</label>
		<label>
			Item 3
			<input list="items" type="text" bind:value={item_3} />
		</label>
		<label>
			Item 4
			<input list="items" type="text" bind:value={item_4} />
		</label>
		<label>
			Item 5
			<input list="items" type="text" bind:value={item_5} />
		</label>
		<datalist id="items">
			{#each itemList as item}
				<option value={item}></option>
			{/each}
		</datalist>
	</fieldset>
	<p>Shop gems</p>
	<fieldset class="input-area">
		<label>
			Gem 1
			<input list="gems" type="text" bind:value={gem_1} />
		</label>
		<label>
			Gem 2
			<input list="gems" type="text" bind:value={gem_2} />
		</label>
		<label>
			Gem 3
			<input list="gems" type="text" bind:value={gem_3} />
		</label>
		<label>
			Gem 4
			<input list="gems" type="text" bind:value={gem_4} />
		</label>
		<datalist id="gems">
			<option value="Opal"></option>
			<option value="Sapphire"></option>
			<option value="Ruby"></option>
			<option value="Garnet"></option>
			<option value="Emerald"></option>
		</datalist>
	</fieldset>
	<button on:click={get_seed_data}>Search</button>
</section>

<h2>Results</h2>
{#if found_seeds.length > 0}
	<div class="seed-list">
		{#each found_seeds as seed}
			<h3>Seed {seed.id}</h3>
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
	<p>No seeds found...</p>
{/if}

<style>
	.input-area {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
