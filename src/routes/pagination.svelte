<script lang="ts">
	import { Pagination } from 'bits-ui';
	import IconCaretRightBold from 'phosphor-icons-svelte/IconCaretRightBold.svelte';
	import IconCaretLeftBold from 'phosphor-icons-svelte/IconCaretLeftBold.svelte';

	type Props = Pagination.RootProps & {
		page: number;
		count: number;
	};

	let { page = $bindable(), count = $bindable(), ...restprops }: Props = $props();
</script>

<Pagination.Root {...restprops} bind:page {count} class="pagination-root">
	{#snippet children({ pages, range })}
		<div class="pagination-entries">
			<Pagination.PrevButton class="pagination-next light-button">
				<IconCaretLeftBold />
			</Pagination.PrevButton>
			{#each pages as page (page.key)}
				{#if page.type === 'page'}
					<Pagination.Page class="pagination-page" {page} />
				{:else if page.type === 'ellipsis'}
					<div class="pagination-ellipsis">...</div>
				{/if}
			{/each}
			<Pagination.NextButton class="pagination-previous light-button">
				<IconCaretRightBold />
			</Pagination.NextButton>
		</div>
		<p>
			Seeds {range.start}&ndash;{range.end} of {count}
		</p>
	{/snippet}
</Pagination.Root>

<style>
	.pagination-entries {
		display: flex;
		gap: var(--size-2);
		justify-content: center;
	}

	:global(.pagination-next),
	:global(.pagination-previous) {
		aspect-ratio: 1 / 1;
	}

	:global(.pagination-next),
	:global(.pagination-previous),
	:global(.pagination-page) {
		width: fit-content;
		padding: 0.5rem 0.75rem;
	}

	.pagination-ellipsis {
		align-self: center;
	}

	p {
		color: var(--text-2);
		text-align: center;
	}
</style>
