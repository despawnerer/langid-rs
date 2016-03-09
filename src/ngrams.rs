use std;


pub fn ngrams(text: &str, length: usize) -> NGrams {
    NGrams {
        length: length,
        chars: text.chars(),
        last_ngram: String::with_capacity(length*2),
    }
}

pub struct NGrams<'a> {
    length: usize,
    chars: std::str::Chars<'a>,
    last_ngram: String,
}


impl<'a> Iterator for NGrams<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.last_ngram.len() < self.length {
            while self.last_ngram.len() < self.length {
                let new_char_option = self.chars.next();
                if new_char_option == None {
                    return None;
                }

                self.last_ngram.push(new_char_option.unwrap())
            }
            return Some(self.last_ngram.clone());
        }

        let new_char_option = self.chars.next();
        if new_char_option == None {
            return None;
        }

        let ngram = self.build_new_ngram(new_char_option.unwrap());
        self.last_ngram = ngram.clone();
        Some(ngram)
    }
}

impl<'a> NGrams<'a> {
    fn build_new_ngram(&mut self, new_character: char) -> String {
        let mut ngram = String::with_capacity(self.length*2);
        let current_chars = self.last_ngram.chars().skip(1);

        for character in current_chars {
            ngram.push(character);
        }
        ngram.push(new_character);

        ngram
    }
}
