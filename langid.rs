
fn main() {
    // let min_n = 1;
    // let max_n: usize = 5;
    // let text = "Estimates of the number of languages in the world vary between 5,000 and 7,000. However, any precise estimate depends on a partly arbitrary distinction between languages and dialects. Natural languages are spoken or signed, but any language can be encoded into secondary media using auditory, visual, or tactile stimuli – for example, in graphic writing, braille, or whistling. This is because human language is modality-independent. Depending on philosophical perspectives regarding the definition of language and meaning, when used as a general concept, 'language' may refer to the cognitive ability to learn and use systems of complex communication, or to describe the set of rules that makes up these systems, or the set of utterances that can be produced from those rules. All languages rely on the process of semiosis to relate signs to particular meanings. Oral and sign languages contain a phonological system that governs how symbols are used to form sequences known as words or morphemes, and a syntactic system that governs how words and morphemes are combined to form phrases and utterances.";
    let text = "Estimates of the number of languages in the world";
    for n in 1..6 {
        for x in ngrams(text, n) {
            println!("{:?}", x);
        }
    }
}

fn ngrams(text: &str, n: usize) -> NGrams {
    NGrams {
        n: n,
        chars: text.chars(),
        last_ngram: String::with_capacity(n*2),
    }
}

struct NGrams<'a> {
    n: usize,
    chars: std::str::Chars<'a>,
    last_ngram: String,
}


impl<'a> Iterator for NGrams<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.last_ngram.len() < self.n {
            while self.last_ngram.len() < self.n {
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
    fn build_new_ngram(&mut self, new_char: char) -> String {
        let mut ngram = String::with_capacity(self.n*2);
        let current_chars = self.last_ngram.chars().skip(1);

        for char in current_chars {
            ngram.push(char);
        }
        ngram.push(new_char);

        ngram
    }
}
