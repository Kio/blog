<script lang='ts'>
	import Post from '$lib/Post.svelte'

	export let data
	const { id } = data.props
</script>

<Post {id}/>
