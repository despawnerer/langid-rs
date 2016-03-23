langid-rs
=========

[![Build Status](https://travis-ci.org/despawnerer/langid-rs.svg?branch=master)](https://travis-ci.org/despawnerer/langid-rs)

NGram-based text classifier written in Rust.

This is not fully ready for use because it lacks pre-trained models and proper documentation.


Usage
-----

### Classifying using pre-trained models

Use the [glob](https://crates.io/crates/glob) crate to get a list of files. Filenames will be used as names for models.

```rust
extern crate langid;
extern crate glob;

use langid::Classifier;
use glob::glob;


fn main() {
	let paths = glob("./language_profiles/*.json").unwrap().filter_map(Result::ok);
	let classifier = Classifier::from_files(paths);

    let language = classifier.classify("Sample text that you want classified.");
    println!("Sample language: {}", language);
}
```


### Training and classifying on the fly

```rust
extern crate langid;

use langid::Classifier;


fn main() {
	let first_language_training_text = "...";
	let second_language_training_text = "...";

	let mut classifier = Classifier::new();
    classifier.train(first_language_training_text, "first");
    classifier.train(second_language_training_text, "second");

    let language = classifier.classify("Sample in the first language.");
    println!("Sample language: {}", language);
}
```


### Training

Run `cargo install langid` to get the `langid` CLI utility.

	langid train [-o FILE] <FILE FILE...>

Create a model based on input text files. Write to stdout or to the file specified by `-o` or `--output`.


Credits
-------

Implements algorithm described by William B. Cavnar and John M. Trenkle, “N-Gram-Based Text Categorization”, 1994.
