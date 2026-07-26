<script lang="ts">
	import IconArrowFatRightFill from 'phosphor-icons-svelte/IconArrowFatRightFill.svelte';
	import IconShareBold from 'phosphor-icons-svelte/IconShareBold.svelte';
	import {
		id_to_icon,
		area_to_icon,
		starting_area_to_icon,
		area_to_name,
		id_to_gem_icon,
		id_to_potion_icon,
		area_to_color,
		type GemName,
		type SeedData
	} from '$lib/item-map';
	import { Seed, type Chest } from '$lib/seed';
	import { Tooltip } from 'bits-ui';
	import BnyTooltip from './bny-tooltip.svelte';
	import { toast } from '@zerodevx/svelte-toast';
	import { difficulty_to_icon, type Settings } from '$lib/util';
	import { Unlocks, StartingArea } from 'rnssp-wasm';

	type Props = {
		seedData: SeedData;
		playerCount: number;
		settings: Settings;
		compact: boolean;
	};

	let {
		seedData,
		playerCount = $bindable(),
		settings,
		compact
	}: Props = $props();

	const seed = $derived(new Seed(seedData));

	async function copySeedLink() {
		const url = new URL(window.location.href);
		url.searchParams.set('seed', String(seed.id));
		if (settings.difficulty !== 3) {
			url.searchParams.set('settings.difficulty', String(settings.difficulty));
		}
		if (settings.starting_area !== StartingArea.RandomKingdom) {
			url.searchParams.set('area', String(settings.starting_area));
		}
		if (!settings.unlocks.is_full()) {
			url.searchParams.set('unlocks', settings.unlocks.get_bitstring());
		}

		const possibleBarColors: GemName[] = ['white', 'opal', 'sapphire', 'ruby', 'garnet', 'emerald'];
		const barColor = possibleBarColors[Math.floor(Math.random() * possibleBarColors.length)];

		if ('clipboard' in navigator) {
			await navigator.clipboard.writeText(url.toString());

			toast.push('Copied seed to clipboard!', {
				theme: {
					'--toastBackground': `var(--surface-${barColor})`,
					'--toastBarBackground': `var(--color-${barColor})`
				}
			});
		} else {
			toast.push("Your browser doesn't support copying to the clipboard...");
		}
	}
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
		<p data-gem={area_to_color(name)}>{area_to_name(name)}</p>
	</div>
{/snippet}

{#snippet chest(index: number, chest: Chest | undefined, areaName: string | undefined = undefined)}
	<div class="chest-label-bar">
		<p class="chest-label">{areaName ?? `Chest ${index}`}</p>
		<p class="chest-color-label">
			<span data-gem={chest?.name}
				>{chest?.label} chest&thinsp;{@render inlineIcon(
					`images/jewels/spr_item_jewels_${chest?.spriteId}.png`
				)}
			</span>
		</p>
	</div>
	<div
		class="chest"
		style={chest?.colorId !== undefined
			? `--chest-background: var(--surface-${seed.chest(index)?.name}); --chest-color: var(--color-${seed.chest(index)?.name})`
			: null}
	>
		{#each chest?.items as item}
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

{#snippet shop(index: number, area: string | undefined = undefined)}
	<div class="shop-label-bar">
		<p class="shop-label">{area_to_name(area) ?? `Shop ${index}`}</p>
	</div>
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
						{#if !compact}
							<img
								width="135"
								height="135"
								class="item-icon"
								src={`images/potions/${id_to_potion_icon(potion.id)}.png`}
								alt="Potion icon"
							/>
						{/if}
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

<article class="seed-entry" class:compact>
	<header>
		<img class="inline-icon difficulty" src={"images/difficulty/" + difficulty_to_icon(settings.difficulty)} />
		<h3>
		<img class="inline-icon" src={"images/starting_area/" + starting_area_to_icon(settings.starting_area)} />
			Seed {seed.id} ({playerCount}p)</h3>
		<Tooltip.Provider delayDuration={0} disableCloseOnTriggerClick={false}>
			<BnyTooltip triggerProps={{ class: 'share-button-root blank-button' }}>
				{#snippet trigger()}
					<button class="share-button light-button" onclick={copySeedLink}>
						<IconShareBold />
					</button>
				{/snippet}
				{#snippet children()}
					<p>Copy seed URL</p>
				{/snippet}
			</BnyTooltip>
		</Tooltip.Provider>
	</header>
	<h4>
		Areas {@render inlineIcon(`images/areas/${area_to_icon('extra_moonlit_prescipice')}.webp`)}
	</h4>
	<div class="area-list">
		{@render area(seed.areaName(0))}
		<IconArrowFatRightFill class="area-arrow" />
		{@render area(seed.areaName(1))}
		<IconArrowFatRightFill class="area-arrow" />
		{@render area(seed.areaName(2))}
		<IconArrowFatRightFill class="area-arrow" />
		{@render area(seed.areaName(3))}
		<IconArrowFatRightFill class="area-arrow" />
		{@render area(seed.areaName(4))}
	</div>
	<h4>Chests {@render inlineIcon('images/toybox.png')}</h4>
	<div class="chest-list">
		{@render chest(0, seed.chest(0), 'Outskirts 1')}
		{@render chest(1, seed.chest(1), 'Outskirts 2')}
		{@render chest(2, seed.chest(2, playerCount), seed.areaTitle(1))}
		{@render chest(3, seed.chest(3, playerCount), seed.areaTitle(2))}
		{@render chest(4, seed.chest(4, playerCount), seed.areaTitle(3))}
		{@render chest(5, seed.chest(5, playerCount), seed.areaTitle(4))}
	</div>
	<h4>Shops {@render inlineIcon('images/coin.png')}</h4>
	{@render shop(0, seed.areaName(1))}
	{@render shop(1, seed.areaName(2))}
	{@render shop(2, seed.areaName(3))}
	{@render shop(3, seed.areaName(4))}
</article>

<style>
	header {
		position: relative;
	}

	h3 {
		text-align: center;
		max-inline-size: unset;
	}

	:global(.share-button-root) {
		position: absolute;
		top: 0;
		right: 0;
		width: unset;
	}

	.share-button {
		padding: var(--size-2);
		font-size: var(--font-size-3);
		color: var(--text-2);
	}

	.difficulty {
  	// margin: 0.5em;
  	height: 2em !important;
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

	.seed-entry header {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
	}

	.chest-label,
	.shop-label {
		margin-block: var(--size-2) var(--size-1);
	}

	.chest-label-bar {
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
			z-index: 0;
		}
	}

	.shop-top-list {
		display: grid;
		grid-template-columns: 1fr 2fr;
		border-bottom: 2px solid var(--surface-3);
		position: relative;
		z-index: 1;
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
		position: relative;
		z-index: 1;
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
			/* I don't know how to do this on mobile */
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

	.compact {
		&.seed-entry {
			padding: var(--size-1) var(--size-2);
		}

		h3 {
			font-size: var(--font-size-4);
		}
		h4 {
			font-size: var(--font-size-2);
			margin-block: var(--size-1) 0;
		}

		& img:not(.coin) {
			display: none;
		}

		:global(.area-arrow) {
			font-size: 1.25rem;
		}

		.chest-label,
		.chest-color-label,
		.shop-label {
			font-size: var(--font-size-1);
			margin: 0;
		}

		.chest {
			margin-bottom: var(--size-1);
			padding: var(--size-1);
		}

		.permanent-items,
		.potion-items,
		.gem-list {
			padding: var(--size-1) var(--size-2);
		}

		.item-text p {
			display: inline;
		}
	}
</style>
