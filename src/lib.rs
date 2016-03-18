mod classifier;
mod model;
mod ngrams;

pub mod utils;

pub use classifier::Classifier;
pub use model::Model;

#[cfg(test)] mod tests;
