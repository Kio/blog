<script lang='ts'>
	import { useQuery, useMutation, useQueryClient } from '@sveltestack/svelte-query'
	import PostExcerpt from '$lib/PostExcerpt.svelte'

	export let slug

	let queryResult = null
	$: queryResult = useQuery(['tag', slug], () =>
		 fetch(`${import.meta.env.VITE_BACKEND_URL}/tags/${slug}`).then(res =>
			 res.json()
		 )
	)
</script>

<div>
	<a href='/'>Back to the Blog</a>
	{#if !$queryResult || $queryResult.isLoading}
		<span>Loading...</span>
	{:else if $queryResult.error}
		<span>An error has occurred: {$queryResult.error.message}</span>
	{:else}
		{#each [$queryResult.data] as {tag, posts}}
			<h1>{tag.name}</h1>
			{#each posts as post}
				<PostExcerpt post={post}/>
			{/each}
		{/each}
	{/if}
</div>
