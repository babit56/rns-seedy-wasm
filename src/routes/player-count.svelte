<script lang="ts">
	import { RatingGroup, Tooltip, type WithoutChildrenOrChild } from 'bits-ui';
	import BnyTooltip from './bny-tooltip.svelte';

	let {
		value = $bindable(0),
		ref = $bindable(null),
		...restProps
	}: WithoutChildrenOrChild<RatingGroup.RootProps> = $props();

	const max_players = 4;

	const bnuuyColors = ['ruby', 'emerald', 'sapphire', 'garnet'];
	const bnuuyQuote = ['...', 'A duet!', 'Rabbits strong together!', 'To me, my allies!'];

	function jumpy(newValue: number) {
		jumping = false;

		// Wait a frame so that the animation is canceled
		requestAnimationFrame(() => {
			jumping = true;
			clearTimeout(jumpingTimeout);
			jumpingTimeout = setTimeout(
				() => {
					jumping = false;
				},
				250 + 60 * newValue
			) as unknown as number; // lmao
		});
	}

	let jumping = $state(false);
	let jumpingTimeout: number | undefined = undefined;
</script>

<div class="player-count">
	<p>Player Count</p>
	<RatingGroup.Root
		bind:value
		bind:ref
		min={1}
		max={max_players}
		{...restProps}
		onValueChange={jumpy}
		class="player-count-group"
	>
		{#snippet children({ items })}
			<Tooltip.Provider delayDuration={0} disableCloseOnTriggerClick={true}>
				{#each items as item (item.index)}
					<RatingGroup.Item index={item.index}>
						<BnyTooltip triggerProps={{ style: `all: unset; ` }}>
							{#snippet trigger()}
								<img
									src={item.state === 'inactive'
										? 'images/rabbit.svg'
										: 'images/rabbit_and_steel.svg'}
									alt="rabbit"
									class="player"
									class:inactive={item.state === 'inactive'}
									class:jumping={jumping && item.index < value}
									style={`--bnuuy-color: var(--color-${bnuuyColors.at(item.index)}); --bnuuy-index: ${value - item.index}`}
								/>
							{/snippet}
							{#snippet children()}
								<div
									class="bnuuy-quote"
									style={`--bnuuy-color: var(--color-${bnuuyColors.at(item.index)})`}
								>
									<Tooltip.Arrow style="color: var(--bnuuy-color)" />
									<p>
										{bnuuyQuote.at(item.index)}
									</p>
								</div>
							{/snippet}
						</BnyTooltip>
					</RatingGroup.Item>
				{/each}
			</Tooltip.Provider>
		{/snippet}
	</RatingGroup.Root>
</div>

<style>
	p {
		text-align: right;
	}
	.player-count {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	:global(.player-count-group) {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.5rem;
		width: fit-content;
	}

	.player {
		height: 40px;
		user-drag: none;

		-webkit-user-drag: none;
		user-select: none;
		-moz-user-select: none;
		-webkit-user-select: none;
		-ms-user-select: none;

		&:hover {
			filter: drop-shadow(0 0 4px var(--bnuuy-color));
		}
		&:hover:active {
			filter: drop-shadow(0 0 2px var(--bnuuy-color));
		}
	}

	.jumping {
		animation: jumpy 250ms calc(60ms * var(--bnuuy-index)) linear;
	}

	.inactive {
		filter: brightness(0.5);
	}

	@keyframes jumpy {
		0% {
			translate: 0 0;
		}
		20% {
			translate: 0 -4px;
		}
		50% {
			translate: 0 -5px;
		}
		60% {
			translate: 0 -5px;
		}
		100% {
			translate: 0 0;
		}
	}

	.bnuuy-quote {
		color: var(--bnuuy-color);
	}
</style>
