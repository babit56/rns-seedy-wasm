<script lang="ts">
	import { Combobox, type WithoutChildrenOrChild, mergeProps } from 'bits-ui';
	import IconCaretDownBold from 'phosphor-icons-svelte/IconCaretDownBold.svelte';
	import Fuse from 'fuse.js';
	import { browser } from '$app/environment';

	type Props = Combobox.RootProps & {
		inputProps?: WithoutChildrenOrChild<Combobox.InputProps>;
		contentProps?: WithoutChildrenOrChild<Combobox.ContentProps>;
	};

	let {
		items = [],
		value = $bindable(),
		open = $bindable(false),
		inputProps,
		contentProps,
		type,
		...restProps
	}: Props = $props();

	let searchValue = $state('');

	const filteredItems = $derived.by(() => {
		if (searchValue === '') return items;
		return fuse.search(searchValue).map((res) => res.item);
	});

	function handleInput(e: Event & { currentTarget: HTMLInputElement }) {
		searchValue = e.currentTarget.value;
		value = searchValue;
	}

	const fuse = new Fuse(items, { keys: ['label'] });

	const mergedRootProps = $derived(mergeProps(restProps));
	const mergedInputProps = $derived(mergeProps(inputProps, { oninput: handleInput }));

	let inputRef = $state<HTMLInputElement | null>(null);
</script>

<Combobox.Root
	{type}
	{items}
	bind:value={value as never}
	bind:open
	disabled={!browser}
	{...mergedRootProps}
>
	<div class="combobox-input-container">
		<Combobox.Input
			clearOnDeselect={true}
			class="combobox-input"
			spellcheck="false"
			bind:ref={inputRef}
			{...mergedInputProps}
		/>
		<Combobox.Trigger class="combobox-trigger" tabindex={-1}>
			<IconCaretDownBold />
		</Combobox.Trigger>
	</div>
	<Combobox.Portal>
		<Combobox.Content class="combobox-content" {...contentProps} side="bottom">
			<Combobox.Viewport class="combobox-viewport">
				{#each filteredItems as item, i (i + item.value)}
					<Combobox.Item class="combobox-item" {...item}>
						{#snippet children({ selected })}
							{item.label}
							{selected ? '✅' : ''}
						{/snippet}
					</Combobox.Item>
				{:else}
					<em>No results found</em>
				{/each}
			</Combobox.Viewport>
		</Combobox.Content>
	</Combobox.Portal>
</Combobox.Root>

<style>
	.combobox-input-container {
		position: relative;
		display: flex;
		flex-grow: 1;
	}

	:global(.combobox-input) {
		width: 100%;
		border-top-right-radius: 0;
		border-bottom-right-radius: 0;
	}

	:global(.combobox-trigger) {
		width: unset;
		padding: var(--size-2);
		background-color: var(--surface-3);
		border-top-left-radius: 0;
		border-bottom-left-radius: 0;
	}

	:global(.combobox-content) {
		background: var(--surface-2);
		min-width: var(--bits-combobox-anchor-width);
		max-height: min(var(--bits-combobox-content-available-height), 300px);
		box-shadow: var(--shadow-5);
	}

	:global(.combobox-viewport) {
		padding: var(--size-1);
		&[data-combobox-viewport] {
			scrollbar-width: thin !important;
		}
	}

	:global(.combobox-item) {
		padding: var(--size-1) var(--size-2);
		display: flex;
		justify-content: space-between;
		cursor: pointer;
		transition: background-color 100ms ease-out;
		&:hover,
		&[data-highlighted] {
			background-color: color-mix(in lch, var(--surface-2), rgb(from var(--surface-4) r g b) 20%);
		}
	}
</style>
