use std::str::CharIndices;
use std::iter::{self, Skip, Zip, Once, Chain};

pub fn ngrams(text: &str, length: usize) -> NGramIterator {
    NGramIterator {
        text: text,
        windows: text.char_indices().zip(text.char_indices()
                                             // Fake "end" char. There are
                                             // probably more efficient ways to
                                             // do this but this works.
                                             .chain(iter::once((text.len(), 0 as char)))
                                             .skip(length)),
    }
}

pub struct NGramIterator<'a> {
    text: &'a str,
    // Some day, this kind of type won't be necessary. Some day...
    windows: Zip<CharIndices<'a>, Skip<Chain<CharIndices<'a>, Once<(usize, char)>>>>,
}


impl<'a> Iterator for NGramIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.windows.next().map(|((a, _), (b, _))| &self.text[a..b])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
    fn longer_than_string() {
        let text = "String";
        let sevengrams: Vec<_> = ngrams(text, 7).collect();
        assert_eq!(sevengrams.len(), 0);
    }
}
