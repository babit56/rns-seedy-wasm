<script lang="ts">
	const spinOffsetSync = (Date.now() % 8000) / 8000;
</script>

<div class="wizard-ko-root">
	<img
		width="315"
		height="478"
		src="images/flight_ring.png"
		style={`--spin-offset: ${spinOffsetSync}`}
		alt="Flight Ring"
		class="flight-ring"
	/>
	<img
		src="images/wizard_hit.png"
		width="500"
		height="500"
		alt="Wizard Rabbit hit"
		class="wizard wizard-hit"
	/>
	<img
		src="images/wizard_ko.png"
		width="500"
		height="500"
		alt="Wizard Rabbit knocked out"
		class="wizard wizard-ko"
	/>
</div>

<style>
	.wizard-ko-root {
		display: grid;
		place-items: center;
		min-height: 100px;
		min-width: 100px;
	}

	.wizard {
		grid-area: 1 / 1 / 1 / 1;
		width: 120px;
		height: 120px;
		position: absolute;
		z-index: 0;
	}

	.wizard-hit {
		opacity: 0;
		animation:
			hit 800ms linear,
			hit-color 800ms linear;
	}

	.wizard-ko {
		display: block;
		animation:
			start-hidden 800ms linear,
			float 4800ms 400ms cubic-bezier(0.37, 0, 0.63, 1) infinite;
	}

	.flight-ring {
		width: 100px;
		height: 100px;
		position: absolute;
		animation: spin 8000ms linear infinite;
		translate: 0 4px;
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
