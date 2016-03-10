use profile::Profile;
use std::collections::HashMap;


pub struct Classifier {
    profiles: HashMap<String, Profile>
}


impl Classifier {
    pub fn new() -> Classifier {
        Classifier { profiles: HashMap::new() }
    }

    pub fn add_profile_from_text(&mut self, text: &str, language: &str) {
        let profile = Profile::build_from_text(text);
        self.profiles.insert(language.to_string(), profile);
    }

    pub fn add_profile_from_file(&mut self, filename: &str, language: &str) {
        let profile = Profile::load_from_file(filename);
        self.profiles.insert(language.to_string(), profile);
    }

    pub fn classify(&self, text: &str) -> &String {
        let profile = Profile::build_from_text(text);
        let (language, matched_profile) = find_min_by_key(
            self.profiles.iter(),
            |&(language, other_profile)| profile.compare(other_profile)
        ).unwrap();
        language
    }
}


fn find_min_by_key<T, I, F>(iterable: I, key: F) -> Option<T>
    where I : IntoIterator<Item=T>,
          F : Fn(&T) -> usize {

    let mut min_value = None;
    let mut min_item = None;
    for item in iterable {
        let value = key(&item);
        if min_value == None || value < min_value.unwrap() {
            min_value = Some(value);
            min_item = Some(item);
        }
    }
    min_item
}
