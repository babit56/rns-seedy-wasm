<script lang="ts">
	import { Switch, Label, useId, type WithoutChildrenOrChild } from 'bits-ui';

	let {
		id = useId(),
		checked = $bindable(true),
		ref = $bindable(null),
		labelText,
		...restProps
	}: WithoutChildrenOrChild<Switch.RootProps> & {
		labelText: string;
	} = $props();
</script>

<div class="switch">
	<Label.Root for={id}><p>{labelText}</p></Label.Root>
	<Switch.Root bind:checked bind:ref {id} {...restProps} class="switch-root">
		<Switch.Thumb class="switch-thumb" />
	</Switch.Root>
</div>

<style>
	.switch {
		display: flex;
		align-items: center;
		gap: var(--size-2);
	}

	:global(:root .switch-root) {
		width: 3.25em;
		height: 2rem;
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
		width: calc(2rem - 6px);
		height: calc(2rem - 6px);
		background-color: var(--text-2);
		border-radius: var(--radius-round);
		transition:
			translate 100ms ease-in-out,
			box-shadow 100ms linear;
		flex-shrink: 0;

		&[data-state='checked'] {
			translate: calc(3.25rem - 2rem - 2px) 0;
			box-shadow:
				0 0 8px hsl(var(--shadow-color) / 50%),
				0 0 4px hsl(var(--shadow-color) / 20%);
		}
	}
</style>
