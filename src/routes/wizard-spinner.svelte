<script lang="ts">
	import { browser } from '$app/environment';
	import { area_to_icon, type AreaName } from '$lib/item-map';
	import { currentLoadingCharacter as character } from '$lib/util';

	const areaList: AreaName[] = [
		'extra_outskirts',
		'hw_nest',
		'hw_lighthouse',
		'hw_arsenal',
		'hw_streets',
		'hw_lakeside',
		'extra_pale_keep',
		'extra_moonlit_prescipice'
	];

	const spinOffsetSync = (Date.now() % 8000) / 8000;
</script>

{#if browser}
	<div class="wizard-spinner-root">
		<div class="areas">
			{#each areaList as areaName, index}
				<img
					width="100"
					height="95"
					class="area-icon"
					src={`images/areas/${area_to_icon(areaName)}.webp`}
					style={`--slide-offset: ${index}`}
					alt="Area icon"
				/>
			{/each}
		</div>
		<div class="flight-ring-root">
			<div
				class="flight-ring"
				style={`--spin-offset: ${spinOffsetSync}; --character-color:
${character.color};`}
			></div>
		</div>
		<img
			src={`images/characters/${character.name}_fly.gif`}
			width={character.width}
			height={character.height}
			alt="Rabbit flying"
			class="wizard wizard-fly"
		/>
		<img
			src={`images/characters/${character.name}_spin.gif`}
			width={character.width}
			height={character.height}
			alt="Rabbit spinning"
			class="wizard wizard-spin"
		/>
	</div>
{/if}

<style>
	.wizard-spinner-root {
		display: grid;
		place-items: center;
		min-height: 100px;
		min-width: 100px;
	}

	.wizard {
		grid-area: 1 / 1 / 1 / 1;
		scale: calc(120 / 500);
		position: absolute;
		z-index: 0;
	}

	.wizard-fly {
		opacity: 0;
		animation: fly-in 800ms cubic-bezier(0.5, 1, 0.89, 1);
	}

	.wizard-spin {
		display: block;
		animation: start-hidden 800ms linear;
	}

	.flight-ring-root {
		width: 100px;
		height: 100px;
		position: absolute;
		animation: fly-in 800ms cubic-bezier(0.5, 1, 0.89, 1);
	}

	.flight-ring {
		height: 100px;
		width: 100px;
		background-color: var(--character-color);
		translate: 0 13px;
		animation: spin 8000ms linear infinite;
		mask: url('$lib/images/flight_ring.png') 0 0 / 100px 100px;
	}

	.areas {
		grid-area: 1 / 1 / 1 / 1;
		display: grid;
		place-items: center;
		width: 150%;

		overflow-x: clip;
		mask-image: linear-gradient(90deg, transparent, black 30%, black 70%, transparent);
	}

	.area-icon {
		grid-area: 1 / 1 / 1 / 1;
		animation: slide-across 9600ms calc(var(--slide-offset, 0) * 1200ms) linear infinite;
		translate: 175% 0;
	}

	@keyframes slide-across {
		0% {
			translate: 175% 0;
		}
		40%,
		100% {
			translate: -175% 0;
		}
	}

	@keyframes fly-in {
		0% {
			translate: -40px 6px;
			opacity: 0;
		}
		100% {
			translate: 0 0;
			opacity: 1;
		}
	}

	@keyframes spin {
		0% {
			rotate: calc(var(--spin-offset) * 1turn);
		}
		100% {
			rotate: calc(var(--spin-offset) * 1turn + 1turn);
		}
	}

	@keyframes start-hidden {
		0%,
		100% {
			display: none;
		}
	}
</style>
