<script lang="ts">
	import { RatingGroup, Tooltip, type WithoutChildrenOrChild } from 'bits-ui';

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
				300 + 60 * newValue
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
						<Tooltip.Root>
							<Tooltip.Trigger style=" all:unset">
								<img
									src={item.state === 'inactive'
										? 'images/rabbit.svg'
										: 'images/rabbit_and_steel.svg'}
									alt="rabbit"
									class="player"
									class:inactive={item.state === 'inactive'}
									class:jumping={jumping && item.index < value}
									style={`--bnuuy-color: var(--color-${bnuuyColors.at(item.index)}); --bnuuy-index: ${item.index}`}
								/>
							</Tooltip.Trigger>
							<Tooltip.Portal>
								<Tooltip.Content>
									<div
										class="bnuuy-quote"
										style={`--bnuuy-color: var(--color-${bnuuyColors.at(item.index)})`}
									>
										<Tooltip.Arrow style="color: var(--bnuuy-color)" />
										<p>
											{bnuuyQuote.at(item.index)}
										</p>
									</div>
								</Tooltip.Content>
							</Tooltip.Portal>
						</Tooltip.Root>
					</RatingGroup.Item>
				{/each}
			</Tooltip.Provider>
		{/snippet}
	</RatingGroup.Root>
</div>

<style>
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
		transition: scale 200ms ease;

		&:hover {
			scale: 0.95;
			filter: drop-shadow(0 0 4px var(--bnuuy-color));
		}
		&:hover:active {
			scale: 0.925;
			filter: drop-shadow(0 0 2px var(--bnuuy-color));
		}
	}

	.jumping {
		animation: jumpy 300ms calc(60ms * var(--bnuuy-index)) linear;
	}

	.inactive {
		opacity: 0.25;
	}

	@keyframes jumpy {
		0% {
			translate: 0 0;
		}
		15% {
			translate: 0 2px;
		}
		20% {
			translate: 0 2px;
		}
		40% {
			translate: 0 -5px;
		}
		60% {
			translate: 0 -5px;
		}
		80% {
			translate: 0 -3px;
		}
		100% {
			translate: 0 0;
		}
	}

	.bnuuy-quote {
		padding: var(--size-1) var(--size-2);
		background-color: var(--surface-1);
		border: 2px solid var(--surface-3);
		border-radius: var(--size-2);
		line-height: 1;
		color: var(--bnuuy-color);
		animation: slide-in 100ms var(--ease-1) forwards;
	}
	:global([data-state='closed']) {
		animation: slide-out 100ms var(--ease-1) forwards;
	}

	@keyframes slide-in {
		0% {
			opacity: 0;
			translate: 0 4px;
		}
		100% {
			opacity: 1;
			translate: 0 0;
		}
	}
	@keyframes slide-out {
		0% {
			opacity: 1;
			translate: 0 0;
		}
		100% {
			opacity: 0;
			translate: 0 4px;
		}
	}
</style>
