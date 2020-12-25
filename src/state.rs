use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct State {
	pub pearls: u64,
	pub rods: u64,
	pub iteration: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Update {
	pub best_rods: State,
	pub best_pearls: State,
	pub best_both: State,
	pub avg_pearls: f64,
	pub avg_rods: f64,
	pub iterations: u64,
}
