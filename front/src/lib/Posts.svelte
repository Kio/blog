<script lant='ts'>
	import { useQuery, useMutation, useQueryClient } from '@sveltestack/svelte-query'
	import PostExcerpt from '$lib/PostExcerpt.svelte'

	const queryClient = useQueryClient()

	const queryResult = useQuery('posts', () => 
		fetch(import.meta.env.VITE_BACKEND_URL).then(res =>
			res.json()
		)
	)

	const format_datetime = (str) => {
		const date = new Date(str)
		return date.toLocaleString()
	}
</script>

<div>
	{#if $queryResult.isLoading}
		<span>Loading...</span>
	{:else if $queryResult.error}
		<span>An error has occurred: {$queryResult.error.message}</span>
	{:else}
		{#each $queryResult.data as post}
			<PostExcerpt post={post} />
		{/each}
	{/if}
</div>
