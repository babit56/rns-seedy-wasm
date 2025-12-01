<script lang="ts">
	import { area_to_icon, type AreaName } from '$lib/item-map';

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
</script>

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
	<img
		src="images/wizard_fly.gif"
		width="500"
		height="500"
		alt="Wizard Rabbit flying"
		class="wizard wizard-fly"
	/>
	<img
		src="images/wizard_spin.gif"
		width="500"
		height="500"
		alt="Wizard Rabbit spinning"
		class="wizard wizard-spin"
	/>
</div>

<style>
	.wizard-spinner-root {
		display: grid;
		place-items: center;
	}

	.wizard {
		grid-area: 1 / 1 / 1 / 1;
		width: 120px;
		height: 120px;
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

	.areas {
		grid-area: 1 / 1 / 1 / 1;
		display: grid;
		place-items: center;
		width: 150%;
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

	@keyframes start-hidden {
		0%,
		100% {
			display: none;
		}
	}
</style>
