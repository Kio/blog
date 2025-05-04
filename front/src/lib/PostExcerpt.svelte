<script lang='ts'>
	import { getPost, postQueryKey, queryClient } from '$lib/api';

	const format_datetime = (str) => {
		const date = new Date(str)
		return date.toLocaleString()
	}

	function prefetchPost(postId: string) {
		const key = postQueryKey(postId);
		if (!queryClient.getQueryData(key)) {
			queryClient.prefetchQuery(key, () => getPost(postId));
		}
	}
</script>

{#each [$$props.post] as {post, tags}}
	<article>
		<a
			class="postLink"
			sveltekit:prefetch
			href={`/posts/${post.id}`}
			on:mouseenter={() => prefetchPost(post.id)}
		>
			<h2>{post.title}</h2>
			<time datetime='{post.created_at}'>{format_datetime(post.created_at)}</time>
			<p>{post.excerpt}</p>
		</a>
		{#each tags as tag}
			<a sveltekit:prefetch class='tag' href={`/tags/${tag.slug}`}>
				{tag.name}
			</a>
		{/each}
	</article>
{/each}

<style>
	.postLink {
		display: block;
	}
	.tag {
		margin-right: 10px;
	}
	a {
		text-decoration: none;
		color: inherit;
	}
</style>
