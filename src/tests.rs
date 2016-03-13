use ngrams::ngrams;
use model::Model;


//model

#[test]
fn test_serialization_and_deserialization() {
    let model = Model::build_from_text("Testing text for serialization");
    let serialized = model.serialize();
    let deserialized = Model::deserialize(serialized);
    assert_eq!(model.ngram_ranks, deserialized.ngram_ranks);
}


// ngrams

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
    let sevengrams: Vec<_> = ngrams(text, 7).collect();
    let empty_vector: Vec<String> = Vec::new();
    assert_eq!(sevengrams, empty_vector);
}
