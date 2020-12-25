mod java_rng;
mod state;

use crate::{
	java_rng::JavaRNG,
	state::{State, Update},
};
use rand_distr::{Binomial, Distribution};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = postMessage)]
	pub fn post_message(data: JsValue);
}

const UPDATE_EVERY: u64 = 25_000;
const RESEED_EVERY: u64 = 2_500;

const ROUND_TO: f64 = 5.0;

#[wasm_bindgen(start)]
pub fn wasm_main() {
	console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn simulate_stream() {
	let rounding: f64 = 10.0_f64.powf(ROUND_TO);

	let mut best_pearls = State::default();
	let mut best_rods = State::default();
	let mut best_both = State::default();

	let blaze_binomial = Binomial::new(305, 0.5).unwrap();
	let mut blaze_rng = JavaRNG::default();
	let mut total_rods = 0_u64;
	let pearl_binomial = Binomial::new(262, 20.0 / 423.0).unwrap();
	let mut pearl_rng = JavaRNG::default();
	let mut total_pearls = 0_u64;
	let mut iterations = 0_u64;
	loop {
		let pearls = pearl_binomial.sample(&mut pearl_rng);
		total_pearls = total_pearls.saturating_add(pearls);
		let rods = blaze_binomial.sample(&mut blaze_rng);
		total_rods = total_rods.saturating_add(rods);

		if pearls > best_pearls.pearls {
			best_pearls.pearls = pearls;
			best_pearls.rods = rods;
			best_pearls.iteration = iterations;
		}

		if rods > best_rods.rods {
			best_rods.pearls = pearls;
			best_rods.rods = rods;
			best_rods.iteration = iterations;
		}

		if rods >= best_both.rods && pearls >= best_both.pearls {
			best_both.pearls = pearls;
			best_both.rods = rods;
			best_both.iteration = iterations;
		}

		if (iterations % UPDATE_EVERY) == 0 {
			let avg_pearls =
				(((total_pearls as f64) / (iterations as f64)) * rounding).round() / rounding;
			let avg_rods =
				(((total_rods as f64) / (iterations as f64)) * rounding).round() / rounding;
			let update = Update {
				avg_pearls,
				avg_rods,
				best_both,
				best_pearls,
				best_rods,
				iterations,
			};
			post_message(JsValue::from_serde(&update).unwrap());
		}

		if (iterations % RESEED_EVERY) == 0 {
			blaze_rng.reseed();
			pearl_rng.reseed();
		}

		iterations = iterations.saturating_add(1);
	}
}
