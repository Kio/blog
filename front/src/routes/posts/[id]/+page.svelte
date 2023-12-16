<script>
	import { page } from '$app/stores'
	import Post from '$lib/Post.svelte'

	const id = $page.params.id
</script>

<Post id={id}/>
