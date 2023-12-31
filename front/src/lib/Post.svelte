<script lang='ts'>
	import { useQuery, useMutation, useQueryClient } from '@sveltestack/svelte-query'
	import { marked } from 'marked'
	import { markedHighlight } from 'marked-highlight'
	import hljs from 'highlight.js/lib/core'
	import javascript from 'highlight.js/lib/languages/javascript'
	import 'highlight.js/styles/a11y-light.css'

	hljs.registerLanguage('javascript', javascript);

	marked.use(markedHighlight({
		highlight(code, lang, info) {
			const language = hljs.getLanguage(lang) ? lang : 'plaintext';
			return hljs.highlight(code, { language }).value;
		}
	}))

	export let id

	let queryClient = null
	$: queryResult = useQuery([`post`, id], () =>
		 fetch(`${import.meta.env.VITE_BACKEND_URL}/posts/${id}`).then(res =>
			 res.json()
		 )
	)

	const format_datetime = (str) => {
		const date = new Date(str)
		return date.toLocaleString()
	}
</script>

<div>
	<a href='/'>Back to the Blog</a>
	{#if !$queryResult || $queryResult.isLoading}
		<span>Loading...</span>
	{:else if $queryResult.error}
		<span>An error has occurred: {$queryResult.error.message}</span>
	{:else}
		{#each [$queryResult.data] as {post, tags}}
			<article>
				<h2>{post.title}</h2>
				<time datetime='{post.created_at}'>{format_datetime(post.created_at)}</time>
				<p>{@html marked(post.text)}</p>
			</article>
		{/each}
	{/if}
</div>
