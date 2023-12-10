<script lant='ts'>
	import { useQuery, useMutation, useQueryClient } from '@sveltestack/svelte-query'

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
			<article>
				<h2>{post.title}</h2>
				<time datetime='{post.created_at}'>{format_datetime(post.created_at)}</time>
				<p>{post.text}</p>
			</article>
		{/each}
	{/if}
</div>
