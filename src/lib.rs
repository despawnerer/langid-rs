extern crate rustc_serialize;
extern crate itertools;

mod classifier;
mod model;
mod ngrams;
mod errors;

pub mod utils;

pub use classifier::Classifier;
pub use model::Model;
pub use errors::DeserializeError;
