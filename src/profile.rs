extern crate rustc_serialize;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::prelude::*;
use std::fs::File;

use self::rustc_serialize::json;

use ngrams::ngrams;


pub type StringCountPair = (String, usize);
pub struct Profile {
    pub ngram_ranks: HashMap<String, usize>,
}


impl Profile {
    pub fn build_from_text(text: &str) -> Profile {
        let mut ngram_counts = HashMap::new();
        for n in 1..6 {
            for ngram in ngrams(text, n) {
                *ngram_counts.entry(ngram).or_insert(0usize) += 1;
            }
        }

        let mut ngrams_and_counts = vec_from_hashmap(&ngram_counts);
        ngrams_and_counts.sort_by(cmp_counts_reverse);

        let mut ngram_ranks = HashMap::new();
        for (index, item) in ngrams_and_counts.iter().enumerate() {
            ngram_ranks.insert((*item).clone().0, index);
        }

        Profile { ngram_ranks: ngram_ranks }
    }

    pub fn load_from_file(path: &str) -> Profile {
        let mut f = File::open(path).unwrap();
        let mut encoded_profile = String::new();
        f.read_to_string(&mut encoded_profile);
        let ngram_ranks = json::decode(&encoded_profile).unwrap();
        Profile { ngram_ranks: ngram_ranks }
    }

    pub fn save_to_file(&self, path: &str) {
        let encoded_profile = json::encode(&self.ngram_ranks).unwrap();
        let mut f = File::create(path).unwrap();
        f.write_all(encoded_profile.as_bytes()).unwrap();
    }

    pub fn compare(&self, other: &Profile) -> usize {
        let max_difference = other.ngram_ranks.len();
        let mut difference = 0;
        for (ngram, index) in self.ngram_ranks.iter() {
            difference += match other.ngram_ranks.get(ngram) {
                Some(other_index) => get_difference(*index, *other_index),
                None => max_difference,
            }
        }
        difference
    }
}


// useful utils

fn get_difference(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}

fn vec_from_hashmap<K, V> (hashmap: &HashMap<K, V>) -> Vec<(K, V)>
         where K: Eq + Hash + Clone, V: Clone {
    let mut vec: Vec<(K, V)> = Vec::new();
    for (key, value) in hashmap.iter() {
        let item = ((*key).clone(), (*value).clone());
        vec.push(item);
    }
    vec
}

fn cmp_counts_reverse(a: &StringCountPair, b: &StringCountPair) -> Ordering {
    b.1.cmp(&a.1)
}
