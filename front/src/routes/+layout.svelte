<script lang="ts">
	import { QueryClientProvider } from '@sveltestack/svelte-query';
	import { queryClient } from '$lib/api';
</script>

<QueryClientProvider client={queryClient}>
	<slot/>
</QueryClientProvider>

<style>
	:global(body) {
		font-family: "Nunito";
	}
</style>

