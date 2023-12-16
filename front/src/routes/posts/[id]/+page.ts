import type { PageLoad } from './$types'

export const load: PageLoad = ({ params }) => {
	return {
		props: {
			id: params.id
		}
	}
}
