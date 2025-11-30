<script lang="ts">
	import { Switch, Label, useId, type WithoutChildrenOrChild } from 'bits-ui';

	let {
		id = useId(),
		checked = $bindable(false),
		ref = $bindable(null),
		labelText,
		...restProps
	}: WithoutChildrenOrChild<Switch.RootProps> & {
		labelText: string;
	} = $props();
</script>

<div class="switch">
	<Switch.Root bind:checked bind:ref {id} {...restProps} class="switch-root">
		<Switch.Thumb class="switch-thumb" />
	</Switch.Root>
	<Label.Root for={id}>{labelText}</Label.Root>
</div>

<style>
	.switch {
		display: flex;
		align-items: center;
		gap: var(--size-2);
	}

	:global(:root .switch-root) {
		width: 60px;
		height: 36px;
		display: inline-flex;
		padding: 0 2px;
		background-color: color-mix(in lch, var(--text-2), var(--surface-1) 90%);
		border: 2px solid var(--surface-4);
		border-radius: var(--radius-round);
		align-items: center;

		&[data-state='checked'] {
			background-color: color-mix(in lch, var(--text-2), var(--surface-1) 50%);
		}
	}

	:global(.switch-thumb) {
		width: 30px;
		height: 30px;
		background-color: var(--text-2);
		border-radius: var(--radius-round);
		transition:
			translate 100ms ease-in-out,
			box-shadow 100ms linear;
		flex-shrink: 0;

		&[data-state='checked'] {
			translate: 24px 0;
			box-shadow:
				0 0 8px hsl(var(--shadow-color) / 50%),
				0 0 4px hsl(var(--shadow-color) / 20%);
		}
	}
</style>
