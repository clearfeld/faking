use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::cell::RefCell;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;

thread_local! {
  static RNG: RefCell<Pcg64Mcg> = RefCell::new(Pcg64Mcg::from_entropy());
}

lazy_static! {
	static ref SEED: RwLock<Option<u64>> = RwLock::new(None);
}

#[wasm_bindgen]
pub fn set_seed(i: u64) {
	let mut guard = SEED.write().unwrap();
	*guard = Some(i);
	RNG.set(Pcg64Mcg::seed_from_u64(i));
}

#[wasm_bindgen]
pub fn get_seed() -> Option<u64> {
	let guard = SEED.read().unwrap();
	return *guard;
}

pub fn get_value_from_range<T, R>(range: R) -> T
where
	T: SampleUniform,
	R: SampleRange<T>,
{
	RNG.with(|rng| rng.borrow_mut().gen_range(range))
}
