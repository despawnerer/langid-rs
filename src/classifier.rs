use model::Model;


pub struct Classifier {
    models: Vec<(String, Model)>
}


impl Classifier {
    pub fn new() -> Classifier {
        Classifier { models: Vec::new() }
    }

    pub fn train(&mut self, text: &str, name: &str) {
        let model = Model::build_from_text(text);
        self.models.push((name.to_owned(), model));
    }

    pub fn classify(&self, text: &str) -> &String {
        let model = Model::build_from_text(text);
        let &(ref name, _) = self.models
            .iter()
            .min_by_key(|&&(_, ref other_model)| model.compare(other_model))
            .unwrap();
        name
    }
}
