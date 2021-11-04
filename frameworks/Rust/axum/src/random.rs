extern crate lazy_static;
extern crate rand;

use std::sync::Arc;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct StandardRandomizerFactory;
struct Incremental(i32);

pub type DynRandomizerFactory = Arc<dyn RandomizerFactory + Send + Sync>;
pub type DynRandomizer = Arc<dyn Randomizer + Send + Sync>;

#[cfg(not(test))]
impl RandomizerFactory for StandardRandomizerFactory  {
    fn create(self) -> DynRandomizer {
        Arc::new(SmallRng::from_entropy()) as DynRandomizer
    }
}

#[cfg(test)]
impl RandomizerFactory for StandardRandomizerFactory  {
    fn create(self) -> DynRandomizer {
        Arc::new(Incremental::new()) as DynRandomizer
    }
}

pub trait RandomizerFactory {
    fn create(self) -> DynRandomizer;
}

pub trait Randomizer {
    fn next(&mut self) -> i32;
}

impl Randomizer for SmallRng {
    fn next(&mut self) -> i32 {
        self.gen_range(1..10000)
    }
}

impl Randomizer for Incremental {
    fn next(&mut self) -> i32 {
        self.0 = self.0 + 1;

        self.0
    }
}

impl Incremental {
    fn new() -> Self {
        Self(0)
    }
}

