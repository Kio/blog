<script lang='ts'>
	import { page } from '$app/stores'
	import Post from '$lib/Post.svelte'

	export let data
</script>

<Post id={data.props.id}/>
