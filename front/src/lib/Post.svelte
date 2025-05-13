<script lang='ts'>
	import { useQuery } from '@sveltestack/svelte-query'
	import { marked } from 'marked'
	import { markedHighlight } from 'marked-highlight'
	import hljs from 'highlight.js/lib/core'
	import javascript from 'highlight.js/lib/languages/javascript'
	import 'highlight.js/styles/a11y-light.css'
	import { getPost, postQueryKey } from '$lib/api'

	hljs.registerLanguage('javascript', javascript);

	// Prevent duplicate registration on every page visit
	if (typeof window !== 'undefined' && !(window as any).__markedConfigured) {
		marked.use(markedHighlight({
			highlight(code, lang, info) {
				const language = hljs.getLanguage(lang) ? lang : 'plaintext';
				return hljs.highlight(code, { language }).value;
			}
		}));
		(window as any).__markedConfigured = true;
	}

	export let id: string
	
	const queryResult = useQuery(postQueryKey(id), () => getPost(id));
	$: data = $queryResult.data;

	const format_datetime = (str) => {
		const date = new Date(str)
		return date.toLocaleString()
	}
</script>

<svelte:head>
	{#if data}
		<title>{data.post.title} | Ivan Konorkin Blog</title>
		<meta name="description" content={data.post.excerpt} />
	{/if}
</svelte:head>

<div>
	<a sveltekit:prefetch href="/">Back to the Blog</a>

	{#if $queryResult.isLoading}
		<span>Loading…</span>
	{:else if $queryResult.error}
		<span>An error has occurred: {$queryResult.error.message}</span>
	{:else}
	<article>
		<h2>{data.post.title}</h2>
		<time datetime={data.post.created_at}>
			{format_datetime(data.post.created_at)}
		</time>
		<div>{@html marked(data.post.text)}</div>
		{#if data.tags?.length}
			<ul>
				{#each data.tags as tag}
					<a sveltekit:prefetch href={`/tags/${tag.slug}`}>
						<li>{tag.name}</li>
					</a>
				{/each}
			</ul>
		{/if}
	</article>
	{/if}
</div>
