<script lant='ts'>
	const format_datetime = (str) => {
		const date = new Date(str)
		return date.toLocaleString()
	}
</script>

{#each [$$props.post] as {post, tags}}
	<article>
		<a href='/posts/{post.id}'>
			<h2>{post.title}</h2>
			<time datetime='{post.created_at}'>{format_datetime(post.created_at)}</time>
			<p>{post.excerpt}</p>
		</a>
		{#each tags as tag}
			<a class='tag' href='/tags/{tag.slug}'>
				{tag.name}
			</a>
		{/each}
	</article>
{/each}

<style>
	.tag {
		margin-right: 10px;
	}
	a {
		text-decoration: none;
		color: inherit;
	}
</style>
