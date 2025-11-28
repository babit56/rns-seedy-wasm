<script lang="ts">
	import IconArrowFatRightFill from 'phosphor-icons-svelte/IconArrowFatRightFill.svelte';
	import { id_to_icon, area_to_icon, area_to_name } from '$lib/item-map';
	import { Seed } from '$lib/seed';
	type Props = {
		seed: Seed;
	};

	let { seed }: Props = $props();
</script>

{#snippet area(name: string)}
	<div class="area">
		{#await import(`$lib/assets/areas/${area_to_icon(name)}.webp`) then { default: src }}
			<img class="area-icon" {src} alt="Area icon" />
		{/await}
		<p>{area_to_name(name)}</p>
	</div>
{/snippet}

{#snippet chest(index: number)}
	<div class="item-list">
		{#each seed.chest(index) as item}
			<div class="item">
				{#await import(`$lib/assets/loot/${id_to_icon(item.id)}.webp`) then { default: src }}
					<img class="item-icon" {src} alt="Loot item" />
				{/await}
				<p>{item.name}</p>
			</div>
		{/each}
	</div>
{/snippet}

{#snippet shop(index: number)}
	<ul>
		<li>
			Potions: {#each seed.shop(index)?.potions as potion, index}
				<span>{potion.name} ({potion.price})</span>{#if index < 2},&nbsp;{/if}
			{/each}
		</li>
		<li>
			Gems: {#each seed.shop(index)?.gems as gem, index}
				<span data-gem={gem.key}>{gem.name} ({gem.price})</span>{#if index < 3},&nbsp;{/if}
			{/each}
		</li>
	</ul>
{/snippet}

{#snippet areaIconSmall(name: string)}
	{#await import(`$lib/assets/areas/${area_to_icon(name)}.webp`) then { default: src }}
		<img class="area-icon-small" {src} alt={area_to_name(name)} />
	{/await}
{/snippet}

<div class="seed-entry">
	<h3>Seed {seed.id}</h3>
	<h4>Areas</h4>
	<div class="area-list">
		{@render area('extra_outskirts')}
		<IconArrowFatRightFill class="area-arrow" />
		{@render area(seed.areaName(0))}
		<IconArrowFatRightFill class="area-arrow" />
		{@render area(seed.areaName(1))}
		<IconArrowFatRightFill class="area-arrow" />
		{@render area(seed.areaName(2))}
		<IconArrowFatRightFill class="area-arrow" />
		{@render area('extra_pale_keep')}
	</div>
	<h4>Chests</h4>
	<div class="chest-list">
		<p class="chest-label">Outskirts 1 {@render areaIconSmall('extra_outskirts')}</p>
		{@render chest(0)}
		<p class="chest-label">Outskirts 2 {@render areaIconSmall('extra_outskirts')}</p>
		{@render chest(1)}
		<p class="chest-label">{seed.areaTitle(0)} {@render areaIconSmall(seed.areaName(0))}</p>
		{@render chest(2)}
		<p class="chest-label">{seed.areaTitle(1)} {@render areaIconSmall(seed.areaName(1))}</p>
		{@render chest(3)}
		<p class="chest-label">{seed.areaTitle(2)} {@render areaIconSmall(seed.areaName(2))}</p>
		{@render chest(4)}
		<p class="chest-label">Pale Keep {@render areaIconSmall('extra_pale_keep')}</p>
		{@render chest(5)}
	</div>
	<h4>Shops</h4>
	<p>{seed.areaTitle(0)} Shop {@render areaIconSmall(seed.areaName(0))}</p>
	{@render shop(0)}
	<p>{seed.areaTitle(1)} Shop {@render areaIconSmall(seed.areaName(1))}</p>
	{@render shop(1)}
	<p>{seed.areaTitle(2)} Shop {@render areaIconSmall(seed.areaName(2))}</p>
	{@render shop(2)}
	<p>Pale Keep Shop {@render areaIconSmall('extra_pale_keep')}</p>
	{@render shop(3)}
</div>

<style>
	h3 {
		text-align: center;
		max-inline-size: unset;
	}

	h4 {
		margin-block: 1em 0.25em;
		padding-bottom: 0.25rem;
		border-bottom: 4px dashed var(--surface-3);
		max-inline-size: unset;
	}

	.seed-entry {
		padding: 1rem;
		background-color: var(--surface-1);
		border: var(--border-size-1) solid var(--surface-4);
		border-radius: var(--radius-2);
		box-shadow: var(--shadow-2);
	}

	.chest-label {
		margin-top: 0.5rem;
	}

	.item-list {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
		gap: 1rem;
		background-color: var(--surface-2);
		padding: var(--size-2);
		border-radius: var(--size-2);
	}

	.area-list {
		display: grid;
		grid-template-columns: repeat(4, minmax(100px, 1fr) auto) minmax(100px, 1fr);
		justify-content: center;
		align-items: center;
		color: var(--text-2);
	}

	.area {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}

	.area-icon {
		width: 100px;
	}

	:global(.area-arrow) {
		font-size: 2rem;
	}

	@media screen and (width < 600px) {
		.area-list {
			display: flex;
			flex-wrap: wrap;
			gap: 1rem;
		}
		.area-icon {
			width: 60px;
		}
		:global(.area-arrow) {
			/* display: none; */
		}
	}

	.item {
		display: flex;
		gap: 0.5rem;
		align-items: center;
		color: var(--text-2);
		text-align: center;
	}

	.item-icon {
		width: 40px;
	}

	.area-icon-small {
		display: inline;
		height: 1.5rem;
		vertical-align: bottom;
	}

	[data-gem='emerald'] {
		color: var(--color-emerald);
	}
	[data-gem='garnet'] {
		color: var(--color-garnet);
	}
	[data-gem='ruby'] {
		color: var(--color-ruby);
	}
	[data-gem='sapphire'] {
		color: var(--color-sapphire);
	}
	[data-gem='opal'] {
		color: var(--color-opal);
	}
</style>
