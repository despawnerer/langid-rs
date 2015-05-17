use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use ngrams::ngrams;


pub type StringCountPair = (String, u64);
pub type LanguageProfile = Vec<StringCountPair>;


pub fn build_from_text(text: &str) -> LanguageProfile {
    let mut ngram_counts = HashMap::new();
    for n in 1..6 {
        for ngram in ngrams(text, n) {
            *ngram_counts.entry(ngram).or_insert(0u64) += 1;
        }
    }

    let mut profile = vec_from_hashmap(&ngram_counts);
    profile.sort_by(cmp_counts_reverse);

    profile
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
