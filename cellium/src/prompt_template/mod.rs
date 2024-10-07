use std::collections::HashMap;

pub struct PromptTemplate {
    templates: HashMap<String, String>,
}

impl PromptTemplate {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    pub fn add_template(&mut self, name: String, template: String) {
        self.templates.insert(name, template);
    }

    pub fn get_template(&self, name: &str) -> Option<&String> {
        self.templates.get(name)
    }
}