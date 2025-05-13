import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import viteCompression from 'vite-plugin-compression';

export default defineConfig({
	plugins: [
		sveltekit(),

		viteCompression({
			algorithm: 'gzip',
			ext: '.gz',
			threshold: 10240,
			deleteOriginFile: false
		}),

		viteCompression({
			algorithm: 'brotliCompress',
			ext: '.br',
			threshold: 10240
		})
	],
});
