pub mod extract_integers;
pub mod graph;

pub use extract_integers::ExtractIntegers;
pub use graph::Graph;

use std::hash::{DefaultHasher, Hash, Hasher};

#[macro_export]
macro_rules! bit {
    ($n:expr) => {
        1 << $n
    };
}

pub fn hash<T: Hash>(thing: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    thing.hash(&mut hasher);
    hasher.finish()
}
