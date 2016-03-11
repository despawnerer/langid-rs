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
        let (name, _) = self.models
            .iter()
            .min_by_key(|&(_, other_model)| model.compare(other_model))
            .unwrap();
        name
    }
}
