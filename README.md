langid-rs
=========

NGram-based text classifier written in Rust.

This is in development and not ready for any kind of use because it lacks any pre-trained models and an easy way to use them.


Usage
-----

### CLI

	langid train [-o FILE] <FILE FILE...>

Create a model based on input text files. Write to stdout or to the file specified by `-o` or `--output`.


### Library

```rust
extern crate langid;

use langid::classifier::Classifier;


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


Credits
-------

William B. Cavnar and John M. Trenkle, “N-Gram-Based Text Categorization”, 1994
