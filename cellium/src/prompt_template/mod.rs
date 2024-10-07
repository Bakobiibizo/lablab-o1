use std::collections::HashMap;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct Template {
    description: String,
    persona: String,
    task: String,
    example: String,
    tools: String,
}

pub struct PromptTemplate {
    templates: HashMap<String, Template>,
}

impl PromptTemplate {
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Load templates from the extra_templates directory
        if let Ok(entries) = fs::read_dir("docs/extra_templates") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("json") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(template) = serde_json::from_str::<Template>(&content) {
                                // Extract template name from filename
                                if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                                    templates.insert(filename.to_string(), template);
                                }
                            }
                        }
                    }
                }
            }
        }

        Self { templates }
    }

    pub fn get_template(&self, name: &str) -> Option<&Template> {
        self.templates.get(name)
    }

    pub fn generate_prompt(&self, content: &str, template_name: &str) -> String {
        if let Some(template) = self.get_template(template_name) {
            // Construct the prompt using the template fields
            format!(
                "{}\n{}\n{}\n{}\n{}\n\n{}",
                template.description,
                template.persona,
                template.task,
                template.example,
                template.tools,
                content
            )
        } else {
            // Fallback prompt
            format!("{}\n\n{}", "Please process the following content:", content)
        }
    }
}