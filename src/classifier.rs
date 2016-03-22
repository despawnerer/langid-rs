use std::path::Path;

use model::Model;
use errors::LoadError;
use utils::read_file;


pub struct Classifier {
    models: Vec<(String, Model)>
}


impl Classifier {
    pub fn new() -> Self {
        Classifier { models: Vec::new() }
    }

    pub fn from_files<P: AsRef<Path>, I: IntoIterator<Item=P>>(paths: I) -> Result<Self, LoadError> {
        let mut classifier = Classifier::new();
        for path in paths {
            let path = path.as_ref();
            let name = path.file_stem().unwrap().to_str().unwrap();
            let mut buf: Vec<u8> = Vec::new();
            try!(read_file(path, &mut buf));
            let model = try!(Model::deserialize(buf));
            classifier.add_model(model, name);
        }
        Ok(classifier)
    }

    pub fn train(&mut self, text: &str, name: &str) {
        let model = Model::build_from_text(text);
        self.add_model(model, name);
    }

    pub fn add_model(&mut self, model: Model, name: &str) {
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
