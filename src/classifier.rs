use std::collections::HashMap;

use model::Model;


pub struct Classifier {
    models: HashMap<String, Model>
}


impl Classifier {
    pub fn new() -> Classifier {
        Classifier { models: HashMap::new() }
    }

    pub fn train(&mut self, text: &str, name: &str) {
        let model = Model::build_from_text(text);
        self.models.insert(name.to_string(), model);
    }

    pub fn classify(&self, text: &str) -> &String {
        let model = Model::build_from_text(text);
        let (name, matched_model) = find_min_by_key(
            self.models.iter(),
            |&(name, other_model)| model.compare(other_model)
        ).unwrap();
        name
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
