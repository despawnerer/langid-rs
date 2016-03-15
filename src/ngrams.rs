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
