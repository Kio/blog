<script lang='ts'>
	import Post from '$lib/Post.svelte'

	export let data

    $: id = data.props.id
</script>

<Post id={id}/>
