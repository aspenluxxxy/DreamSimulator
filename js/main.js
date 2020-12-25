// eslint-disable-next-line no-unused-vars
import wasm from '../Cargo.toml';
import SimWorker from 'web-worker:./simworker';

var worker = null;

// source: https://stackoverflow.com/questions/47879864/how-can-i-check-if-a-browser-supports-webassembly
const supported = (() => {
	try {
		if (
			typeof WebAssembly === 'object' &&
			typeof WebAssembly.instantiate === 'function'
		) {
			const module = new WebAssembly.Module(
				Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00)
			);
			if (module instanceof WebAssembly.Module)
				return (
					new WebAssembly.Instance(module) instanceof
					WebAssembly.Instance
				);
		}
	} catch (e) {}
	return false;
})();

if (supported) {
	document.addEventListener('DOMContentLoaded', function () {
		document.getElementById('start-sim').onclick = function () {
			if (worker !== null) {
				console.log('worker stopped');
				worker.terminate();
			}
			worker = new SimWorker();
			worker.onerror = function (e) {
				console.error(e.message, e);
			};
			worker.onmessage = function (e) {
				document.getElementById(
					'iterations'
				).textContent = e.data.iterations.toLocaleString();

				document.getElementById(
					'avg-pearls'
				).textContent = e.data.avg_pearls.toString();
				document.getElementById(
					'avg-rods'
				).textContent = e.data.avg_rods.toString();

				let pearls = e.data.best_pearls;
				document.getElementById(
					'best-pearls-pearls'
				).textContent = pearls.pearls.toString();
				document.getElementById(
					'best-pearls-rods'
				).textContent = pearls.rods.toString();
				document.getElementById(
					'best-pearls-iteration'
				).textContent = pearls.iteration.toLocaleString();

				let rods = e.data.best_rods;
				document.getElementById(
					'best-blaze-pearls'
				).textContent = rods.pearls.toString();
				document.getElementById(
					'best-blaze-rods'
				).textContent = rods.rods.toString();
				document.getElementById(
					'best-blaze-iteration'
				).textContent = rods.iteration.toLocaleString();

				let overall = e.data.best_both;
				document.getElementById(
					'best-overall-pearls'
				).textContent = overall.pearls.toString();
				document.getElementById(
					'best-overall-rods'
				).textContent = overall.rods.toString();
				document.getElementById(
					'best-overall-iteration'
				).textContent = overall.iteration.toLocaleString();
			};
			worker.onmessageerror = function (e) {
				console.error(e.message, e);
			};
			console.log(worker);
		};

		document.getElementById('stop-sim').onclick = function () {
			if (worker !== null) {
				worker.terminate();
			}
			worker = null;
		};
	});
} else {
	alert('WebAssembly support is needed to use DreamSimulator!');
}
