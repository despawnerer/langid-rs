use ngrams::ngrams;

#[test]
fn unigrams() {
    let text = "Estimates";
    let unigrams: Vec<_> = ngrams(text, 1).collect();
    assert_eq!(unigrams, ["E", "s", "t", "i", "m", "a", "t", "e", "s"]);
}

#[test]
fn bigrams() {
    let text = "Estimates";
    let bigrams: Vec<_> = ngrams(text, 2).collect();
    assert_eq!(bigrams, ["Es", "st", "ti", "im", "ma", "at", "te", "es"]);
}

#[test]
fn trigrams() {
    let text = "Estimates";
    let trigrams: Vec<_> = ngrams(text, 3).collect();
    assert_eq!(trigrams, ["Est", "sti", "tim", "ima", "mat", "ate", "tes"]);
}

#[test]
fn longer_than_string_ngrams() {
    let text = "String";
    let sevengrams: Vec<String> = ngrams(text, 7).collect();
    let empty_vector: Vec<String> = Vec::new();
    assert_eq!(sevengrams, empty_vector);
}
