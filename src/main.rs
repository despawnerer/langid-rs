extern crate langid;

use langid::profile;


fn main() {
    let text = "Estimates of the number of languages in the world vary between 5,000 and 7,000. However, any precise estimate depends on a partly arbitrary distinction between languages and dialects. Natural languages are spoken or signed, but any language can be encoded into secondary media using auditory, visual, or tactile stimuli – for example, in graphic writing, braille, or whistling. This is because human language is modality-independent. Depending on philosophical perspectives regarding the definition of language and meaning, when used as a general concept, 'language' may refer to the cognitive ability to learn and use systems of complex communication, or to describe the set of rules that makes up these systems, or the set of utterances that can be produced from those rules. All languages rely on the process of semiosis to relate signs to particular meanings. Oral and sign languages contain a phonological system that governs how symbols are used to form sequences known as words or morphemes, and a syntactic system that governs how words and morphemes are combined to form phrases and utterances.";
    let profile = profile::build_from_text(text);

    println!("Ranks of each ngram:");
    for (ngram, rank) in profile {
        println!("{}: {}", ngram, rank);
    }
}

