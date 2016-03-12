use std;


pub fn ngrams(text: &str, length: usize) -> NGramIterator {
    NGramIterator {
        length: length,
        chars: text.chars(),
        last_ngram: String::new(),
    }
}

pub struct NGramIterator<'a> {
    length: usize,
    chars: std::str::Chars<'a>,
    last_ngram: String,
}


impl<'a> Iterator for NGramIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut ngram: String = self.last_ngram.chars().skip(1).collect();

        while ngram.len() < self.length {
            match self.chars.next() {
                Some(character) => {
                    ngram.push(character);
                },
                None => {
                    return None;
                },
            };
        }

        self.last_ngram = ngram.clone();
        return Some(ngram);
    }
}
