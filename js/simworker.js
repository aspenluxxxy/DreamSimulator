import wasm from '../Cargo.toml';

(async () => {
	const Sim = await wasm();

	Sim.simulate_stream();
})();
