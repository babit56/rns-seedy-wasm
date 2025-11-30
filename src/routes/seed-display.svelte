<script lang="ts">
	import IconArrowFatRightFill from 'phosphor-icons-svelte/IconArrowFatRightFill.svelte';
	import {
		id_to_icon,
		area_to_icon,
		area_to_name,
		id_to_gem_icon,
		id_to_potion_icon,
		area_to_color,
		type AreaName
	} from '$lib/item-map';
	import { Seed } from '$lib/seed';
	type Props = {
		seed: Seed;
		playerCount: number;
	};

	let { seed, playerCount = $bindable() }: Props = $props();
</script>

{#snippet area(name: string)}
	<div class="area">
		<img
			width="100"
			height="95"
			class="area-icon"
			src={`images/areas/${area_to_icon(name)}.webp`}
			alt="Area icon"
		/>
		<p>{area_to_name(name)}</p>
	</div>
{/snippet}

{#snippet chest(index: number, areaName: string | undefined = undefined)}
	<div class="chest-label">
		<p>{areaName ?? `Chest ${index}`}</p>
		<p>
			{#if seed.chest(index)}
				{@render inlineIcon(
					`images/jewels/spr_item_jewels_${seed.chest(index)?.spriteId}.png`
				)}&thinsp;
			{/if}<span data-gem={seed.chest(index)?.name}>{seed.chest(index)?.label} chest</span>
		</p>
	</div>
	<div
		class="chest"
		style={seed.chest(index)?.colorId !== undefined
			? `--chest-background: var(--surface-${seed.chest(index)?.name}); --chest-color: var(--color-${seed.chest(index)?.name})`
			: null}
	>
		{#each seed.chest(index, playerCount)?.items as item}
			<div class="item">
				<img
					width="110"
					height="110"
					class="item-icon"
					src={`images/loot/${id_to_icon(item.id)}.webp`}
					alt="Loot item"
				/>
				<p class="loot-name">{item.name}</p>
			</div>
		{/each}
	</div>
{/snippet}

{#snippet shop(index: number, area: AreaName | undefined = undefined)}
	<div
		class="shop"
		style={area
			? `--shop-background: var(--surface-${area_to_color(area)}); --shop-color: var(--color-${area_to_color(area)})`
			: null}
	>
		<div class="shop-top-list">
			<div class="permanent-items">
				<div class="item">
					<img
						width="135"
						height="135"
						class="item-icon"
						src="images/Full_heal_square.png"
						alt="Potion icon"
					/>
					<div class="item-text">
						<p>Full Heal</p>
						<p><span class="item-price">{@render coin()} 5</span></p>
					</div>
				</div>
				<div class="item">
					<img
						width="135"
						height="135"
						class="item-icon"
						src="images/Level_up_orb.png"
						alt="Potion icon"
					/>
					<div class="item-text">
						<p>Level Up</p>
						<p><span class="item-price">{@render coin()} 5</span></p>
					</div>
				</div>
			</div>
			<div class="potion-items">
				{#each seed.shop(index)?.potions as potion}
					<div class="item">
						<img
							width="135"
							height="135"
							class="item-icon"
							src={`images/potions/${id_to_potion_icon(potion.id)}.png`}
							alt="Potion icon"
						/>
						<div class="item-text">
							<p class="potion-name">{potion.name}</p>
							<p><span class="item-price">{@render coin()} {potion.price}</span></p>
						</div>
					</div>
				{/each}
			</div>
		</div>
		<div class="gem-list">
			{#each seed.shop(index)?.gems as gem}
				<div class="gem">
					<img
						width="110"
						height="110"
						class="item-icon"
						src={`images/gems/${id_to_gem_icon(gem.id)}.png`}
						alt="Loot item"
					/>
					<div class="item-text">
						<p data-gem={gem.key}>{gem.name}</p>
						<p><span class="item-price">{@render coin()} {gem.price}</span></p>
					</div>
				</div>
			{/each}
		</div>
	</div>
{/snippet}

{#snippet coin()}
	<img width="60" height="60" class="coin" src="images/coin.png" alt="Coin" />
{/snippet}

{#snippet inlineIcon(src: string, alt: string | null = null)}
	<img class="inline-icon" {src} {alt} />
{/snippet}

<div class="seed-entry">
	<h3>Seed {seed.id} ({playerCount}p)</h3>
	<h4>
		Areas {@render inlineIcon(`images/areas/${area_to_icon('extra_moonlit_prescipice')}.webp`)}
	</h4>
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
	<h4>Chests {@render inlineIcon('images/Shop-icon.png')}</h4>
	<div class="chest-list">
		{@render chest(0, 'Outskirts 1')}
		{@render chest(1, 'Outskirts 2')}
		{@render chest(2, seed.areaTitle(0))}
		{@render chest(3, seed.areaTitle(1))}
		{@render chest(4, seed.areaTitle(2))}
		{@render chest(5, 'Pale Keep')}
	</div>
	<h4>Shops {@render inlineIcon('images/coin.png')}</h4>
	<p class="shop-label">{seed.areaTitle(0)}</p>
	{@render shop(0, seed.areaName(0))}
	<p class="shop-label">{seed.areaTitle(1)}</p>
	{@render shop(1, seed.areaName(1))}
	<p class="shop-label">{seed.areaTitle(2)}</p>
	{@render shop(2, seed.areaName(2))}
	<p class="shop-label">Pale Keep</p>
	{@render shop(3, 'extra_pale_keep')}
</div>

<style>
	h3 {
		text-align: center;
		max-inline-size: unset;
	}

	h4 {
		margin-block: 1em 0.25em;
		padding-bottom: 0.25rem;
		border-bottom: 4px solid var(--surface-2);
		max-inline-size: unset;
	}

	.seed-entry {
		padding: 1rem;
		background-color: var(--surface-1);
		border: var(--border-size-1) solid var(--surface-2);
		border-radius: var(--radius-2);
		box-shadow: var(--shadow-4);
	}

	.chest-label,
	.shop-label {
		margin-block: var(--size-2) var(--size-1);
	}

	.chest-label {
		display: flex;
		justify-content: space-between;
	}

	.loot-name {
		line-height: 1.2;
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
		aspect-ratio: 20 / 19;
	}

	:global(.area-arrow) {
		font-size: 2rem;
	}

	.chest {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
		gap: 1rem;
		padding: var(--size-2) var(--size-3);
		background-color: var(--chest-background, var(--surface-2));
		border-color: var(--chest-color, var(--surface-2));
		border-left-width: 2px;
		border-right-width: 2px;
	}

	.item {
		display: flex;
		gap: 0.5rem;
		align-items: center;
		color: var(--text-2);
		overflow-wrap: anywhere;
	}

	.item-icon {
		width: 40px;
		aspect-ratio: 1 / 1;
	}

	.shop {
		background-color: var(--shop-background, var(--surface-2));
		border-color: var(--shop-color, var(--surface-2));
		border-style: dashed;
		border-left-width: 3px;
		border-right-width: 3px;
		position: relative;

		--c: color-mix(in lch, var(--shop-color), transparent 70%);

		&::before {
			content: '';
			position: absolute;
			width: 100%;
			height: 100%;
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
			opacity: 0.2;
			mask-image: linear-gradient(
				-30deg,
				black,
				black 5%,
				transparent 30%,
				transparent 70%,
				black 95%,
				black
			);
		}
	}

	.shop-top-list {
		display: grid;
		grid-template-columns: 1fr 2fr;
		border-bottom: 2px solid var(--surface-3);
	}
	.permanent-items {
		display: grid;
		grid-template-columns: repeat(2, minmax(80px, 1fr));
		border-right: 2px solid var(--surface-3);
		padding: var(--size-2) var(--size-2) var(--size-2) var(--size-3);
	}
	.potion-items {
		display: grid;
		grid-template-columns: repeat(3, minmax(80px, 1fr));
		gap: var(--size-3);
		padding: var(--size-2) var(--size-3) var(--size-2) var(--size-2);
	}

	.gem-list {
		display: grid;
		grid-template-columns: repeat(4, minmax(120px, 1fr));
		gap: 0.75rem;
		padding: var(--size-2) var(--size-3);
	}

	.gem {
		display: flex;
		gap: 0.5rem;
		align-items: center;
		color: var(--text-2);
		overflow-wrap: anywhere;
	}

	.item-text {
		line-height: 1.4;
		font-weight: var(--font-weight-6);
	}

	/* Some potion names are too long... */
	.potion-name {
		font-size: var(--font-size-fluid-0);
	}

	.item-price {
		display: inline-block;
		padding: 0 0.25em 0.15em;
		border-radius: 0.25rem;
		border: 2px solid var(--surface-3);
		background: var(--surface-1);
		line-height: 1.2;
	}

	@media screen and (width < 800px) {
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

		.shop-top-list {
			grid-template-columns: 1fr;
		}
		.permanent-items {
			border-width: 0;
		}
		.potion-items {
			grid-template-columns: repeat(2, minmax(80px, 1fr));
		}
		.gem-list {
			grid-template-columns: repeat(2, minmax(120px, 1fr));
		}
	}

	.coin {
		display: inline;
		width: 1rem;
		aspect-ratio: 1 / 1;
		vertical-align: bottom;
	}

	.inline-icon {
		display: inline;
		height: 1.2em;
		vertical-align: bottom;
	}

	[data-gem='white'] {
		color: var(--color-white);
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
