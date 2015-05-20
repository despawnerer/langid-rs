extern crate rustc_serialize;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::prelude::*;
use std::fs::File;

use self::rustc_serialize::json;

use ngrams::ngrams;


pub type StringCountPair = (String, u64);
pub type LanguageProfile = HashMap<String, usize>;


pub fn build_from_text(text: &str) -> LanguageProfile {
    let mut ngram_counts = HashMap::new();
    for n in 1..6 {
        for ngram in ngrams(text, n) {
            *ngram_counts.entry(ngram).or_insert(0u64) += 1;
        }
    }

    let mut ngrams_and_counts = vec_from_hashmap(&ngram_counts);
    ngrams_and_counts.sort_by(cmp_counts_reverse);

    let mut profile = LanguageProfile::new();
    for (index, item) in ngrams_and_counts.iter().enumerate() {
        profile.insert((*item).clone().0, index);
    }

    profile
}


pub fn save(profile: &LanguageProfile, path: &str) {
    let encoded_profile = json::encode(&profile).unwrap();
    let mut f = File::create(path).unwrap();
    f.write_all(encoded_profile.as_bytes()).unwrap();
}


pub fn load(path: &str) -> LanguageProfile {
    let mut f = File::open(path).unwrap();
    let mut encoded_profile = String::new();
    f.read_to_string(&mut encoded_profile);
    json::decode(&encoded_profile).unwrap()
}


// useful utils

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
