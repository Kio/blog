import { QueryClient } from '@sveltestack/svelte-query';

export const queryClient = new QueryClient();

export async function getPost(id: string) {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/posts/${id}`)
  return res.json()
}

export function postQueryKey(id: string) {
  return ['post', id];
}
