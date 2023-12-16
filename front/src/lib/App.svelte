<script>
	import { QueryClient, QueryClientProvider } from '@sveltestack/svelte-query'
	import Posts from '$lib/Posts.svelte'

	const queryClient = new QueryClient()
</script>

<QueryClientProvider client={queryClient}>
	<Posts />
</QueryClientProvider>

<style>
	:global(body) {
		font-family: "Nunito";
	}
</style>
