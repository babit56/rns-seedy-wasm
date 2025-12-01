<script lang="ts">
	import type { Snippet } from 'svelte';
	import { AlertDialog, type WithoutChild } from 'bits-ui';
	import type { MouseEventHandler } from 'svelte/elements';

	type Props = AlertDialog.RootProps & {
		title: Snippet;
		description: Snippet;
		confirm: string;
		contentProps?: WithoutChild<AlertDialog.ContentProps>;
		onAction?: MouseEventHandler<HTMLButtonElement>;
		// ...other component props if you wish to pass them
	};

	let {
		open = $bindable(false),
		children,
		contentProps,
		title,
		description,
		confirm,
		onAction,
		...restProps
	}: Props = $props();
</script>

<AlertDialog.Root bind:open {...restProps}>
	<AlertDialog.Portal>
		<AlertDialog.Overlay class="dialog-overlay" />
		<AlertDialog.Content class="dialog-content" {...contentProps}>
			<AlertDialog.Title class="dialog-title">
				{@render title()}
			</AlertDialog.Title>
			<AlertDialog.Description class="dialog-description">
				<div class="prose">
					{@render description()}
				</div>
			</AlertDialog.Description>
			{@render children?.()}
			<AlertDialog.Action onclick={onAction}>{confirm}</AlertDialog.Action>
		</AlertDialog.Content>
	</AlertDialog.Portal>
</AlertDialog.Root>

<style>
	:global(.dialog-overlay) {
		position: fixed;
		inset: 0;
		background-color: var(--surface-overlay);
	}

	:global(.dialog-content) {
		position: fixed;
		width: 400px;
		max-width: calc(100vw - 2rem);
		left: 50%;
		top: 50%;
		translate: -50% -50%;
		padding: var(--size-3);

		background-color: var(--surface-2);
		border-radius: var(--size-2);
		box-shadow: var(--shadow-2);
	}

	:global(.dialog-title) {
		text-align: center;
		font-size: var(--font-size-4);
		font-weight: var(--font-weight-6);
		margin-bottom: 1rem;
	}

	:global(.dialog-description) {
		margin-bottom: 1rem;
		color: var(--text-2);
	}
</style>
