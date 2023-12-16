<script lang='ts'>
	export let data

	import Tag from '$lib/Tag.svelte'
</script>

<Tag slug={data.props.slug} />
