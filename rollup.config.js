import del from 'rollup-plugin-delete';
import rust from '@wasm-tool/rollup-plugin-rust';
import serve from 'rollup-plugin-serve';
import webWorkerLoader from 'rollup-plugin-web-worker-loader';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import { terser } from 'rollup-plugin-terser';

const production = !process.env.ROLLUP_WATCH;

export default {
	input: 'js/main.js',
	output: {
		dir: 'web',
		format: 'cjs',
	},
	plugins: [
		del({
			targets: 'web/assets/*',
		}),
		webWorkerLoader({
			targetPlatform: 'browser',
			inline: false,
		}),
		nodeResolve({ browser: true }),
		rust({
			watchPatterns: ['src/**/*'],
		}),
		production && terser(),
		!production &&
			serve({
				contentBase: 'web',
				mimeTypes: {
					'application/wasm': ['wasm'],
				},
			}),
	],
};
