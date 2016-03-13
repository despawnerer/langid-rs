extern crate rustc_serialize;
extern crate itertools;

use std::collections::HashMap;

use self::itertools::Itertools;
use self::rustc_serialize::json;

use ngrams::ngrams;


pub struct Model {
    pub ngram_ranks: HashMap<String, usize>,
}


impl Model {
    pub fn build_from_text(text: &str) -> Model {
        let mut ngram_counts = HashMap::new();
        for n in 1..6 {
            for ngram in ngrams(text, n) {
                *ngram_counts.entry(ngram).or_insert(0usize) += 1;
            }
        }

        let ngrams = ngram_counts
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            .into_iter()
            .map(|(ngram, _count)| ngram);

        let mut ngram_ranks = HashMap::new();
        for (rank, ngram) in ngrams.enumerate() {
            ngram_ranks.insert(ngram, rank);
        }

        Model { ngram_ranks: ngram_ranks }
    }

    pub fn deserialize(bytes: Vec<u8>) -> Model {
        let string = String::from_utf8(bytes).unwrap();
        let ngram_ranks = json::decode(string.as_str()).unwrap();
        Model { ngram_ranks: ngram_ranks }
    }

    pub fn serialize(&self) -> Vec<u8> {
        json::encode(&self.ngram_ranks).unwrap().into_bytes()
    }

    pub fn compare(&self, other: &Model) -> usize {
        let max_difference = other.ngram_ranks.len();
        let mut difference = 0;
        for (ngram, rank) in self.ngram_ranks.iter() {
            difference += match other.ngram_ranks.get(ngram) {
                Some(other_rank) => get_difference(*rank, *other_rank),
                None => max_difference,
            }
        }
        difference
    }
}


fn get_difference(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}
