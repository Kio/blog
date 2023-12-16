<script>
	import { QueryClient, QueryClientProvider } from '@sveltestack/svelte-query'

	const queryClient = new QueryClient()
</script>

<QueryClientProvider client={queryClient}>
	<slot>
		No content
	</slot>
</QueryClientProvider>

<style>
	:global(body) {
		font-family: "Nunito";
	}
</style>

