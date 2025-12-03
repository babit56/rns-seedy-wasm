<script lang="ts">
	import { browser } from '$app/environment';
	import { area_to_icon, type AreaName } from '$lib/item-map';
	import { currentLoadingCharacter as character } from '$lib/util';

	type SpinnerMode = 'idle' | 'error';

	type Props = {
		mode: SpinnerMode;
	};

	let { mode = $bindable('idle') }: Props = $props();

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

	// Sync ring spin position when changing modes
	let spinOffsetSync = $state((Date.now() % 8000) / 8000);
	$effect(() => {
		mode;
		spinOffsetSync = (Date.now() % 8000) / 8000;
	});
</script>

{#if browser}
	<div class="rabbit-root">
		<div class="areas" class:hide={mode === 'error'}>
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
		<div class="flight-ring-root" class:fly-in={mode === 'idle'}>
			<div
				class="flight-ring"
				style={`--spin-offset: ${spinOffsetSync}; --character-color:
${character.color};`}
			></div>
		</div>
		{#if mode === 'idle'}
			<img
				src={`images/characters/${character.name}_fly.gif`}
				width={character.width}
				height={character.height}
				alt="Rabbit flying"
				class="rabbit rabbit-fly"
			/>
			<img
				src={`images/characters/${character.name}_spin.gif`}
				width={character.width}
				height={character.height}
				alt="Rabbit spinning"
				class="rabbit rabbit-spin"
			/>
		{:else if mode === 'error'}
			<img
				src={`images/characters/${character.name}_hit.png`}
				width={character.width}
				height={character.height}
				alt="Rabbit hit"
				class="rabbit rabbit-hit"
			/>
			<img
				src={`images/characters/${character.name}_ko.png`}
				width={character.width}
				height={character.height}
				alt="Rabbit knocked out"
				class="rabbit rabbit-ko"
			/>
		{/if}
	</div>
{/if}

<style>
	.rabbit-root {
		display: grid;
		place-items: center;
		min-height: 100px;
		min-width: 100px;
	}

	.rabbit {
		grid-area: 1 / 1 / 1 / 1;
		width: 120px;
		position: absolute;
		z-index: 0;
	}

	.rabbit-fly {
		opacity: 0;
		animation: fly-in 800ms cubic-bezier(0.5, 1, 0.89, 1);
	}

	.rabbit-spin {
		display: block;
		animation: start-hidden 800ms linear;
	}

	.rabbit-hit {
		opacity: 0;
		animation:
			hit 800ms linear,
			hit-color 800ms linear;
	}

	.rabbit-ko {
		display: block;
		animation:
			start-hidden 800ms linear,
			float 4800ms 400ms cubic-bezier(0.37, 0, 0.63, 1) infinite;
	}

	.flight-ring-root {
		width: 100px;
		height: 100px;
		position: absolute;
		&.fly-in {
			animation: fly-in 800ms cubic-bezier(0.5, 1, 0.89, 1);
		}
	}

	.flight-ring {
		height: 100px;
		width: 100px;
		translate: 0 13px;
		background-color: color-mix(in srgb, var(--character-color) 100%, #fff 50%);
		background-blend-mode: screen;
		opacity: 0.5;
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
		transition:
			display 800ms allow-discrete,
			opacity 800ms ease-in;

		&.hide {
			opacity: 0;
			display: none;
		}
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

	@keyframes hit {
		0% {
			opacity: 1;
		}
		8% {
			translate: -40px 0;
		}
		22% {
			translate: 20px 0;
		}
		40% {
			translate: -5px 0;
		}
		73% {
			translate: 0 0;
			filter: none;
		}
		100% {
			opacity: 1;
		}
	}

	@keyframes hit-color {
		0% {
			filter: grayscale(100%) sepia(100%) hue-rotate(-50deg) brightness(150%) saturate(400%);
		}
		40% {
			filter: grayscale(100%) sepia(100%) hue-rotate(-50deg) brightness(150%) saturate(400%);
		}
	}

	@keyframes float {
		0%,
		100% {
			translate: 0 4px;
		}
		50% {
			translate: 0 -4px;
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
