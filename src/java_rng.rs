use java_random::Random;
use rand::{Error, Rng, RngCore};

pub struct JavaRNG {
	inner: Random,
}

impl JavaRNG {
	pub fn reseed(&mut self) {
		self.inner.set_seed(rand::thread_rng().gen())
	}
}

impl Default for JavaRNG {
	fn default() -> Self {
		Self {
			inner: Random::with_seed(rand::thread_rng().gen()),
		}
	}
}

impl RngCore for JavaRNG {
	fn next_u32(&mut self) -> u32 {
		self.inner.next_int() as u32
	}

	fn next_u64(&mut self) -> u64 {
		(self.next_u32() as u64) << 32 | (self.next_u32() as u64)
	}

	fn fill_bytes(&mut self, _dest: &mut [u8]) {
		unimplemented!()
	}

	fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), Error> {
		unimplemented!()
	}
}
