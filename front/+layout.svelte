<script lang="ts">
	import { QueryClientProvider, QueryClient } from '@sveltestack/svelte-query';

    export const queryClient = new QueryClient();
</script>

<QueryClientProvider client={queryClient}>
	<slot />
</QueryClientProvider>