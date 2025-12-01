<script lang="ts">
	import { Tooltip } from 'bits-ui';
	import { type Snippet } from 'svelte';

	type Props = Tooltip.RootProps & {
		trigger: Snippet;
		triggerProps?: Tooltip.TriggerProps;
		contentProps?: Tooltip.ContentProps;
	};

	let {
		open = $bindable(false),
		children,
		trigger,
		triggerProps = {},
		contentProps = {},
		...restProps
	}: Props = $props();
</script>

<!--
 Ensure you have a `Tooltip.Provider` component wrapping
 your root layout content
-->
<Tooltip.Root bind:open {...restProps}>
	<Tooltip.Trigger {...triggerProps}>
		{@render trigger()}
	</Tooltip.Trigger>
	<Tooltip.Portal>
		<Tooltip.Content class="tooltip-content" {...contentProps}>
			<Tooltip.Arrow />
			{@render children?.()}
		</Tooltip.Content>
	</Tooltip.Portal>
</Tooltip.Root>

<style>
	:global(.tooltip-content) {
		padding: var(--size-1) var(--size-2);
		background-color: var(--surface-1);
		border: 2px solid var(--surface-3);
		border-radius: var(--size-2);
		line-height: 1;
		color: var(--text-2);
		animation: slide-in 100ms var(--ease-1) forwards;
		&[data-state='closed'] {
			animation: slide-out 100ms var(--ease-1) forwards;
		}
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
