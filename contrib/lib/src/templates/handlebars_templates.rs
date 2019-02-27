use templates::serde::Serialize;
use templates::{Engine, TemplateInfo};

pub use templates::handlebars::Handlebars;

impl Engine for Handlebars {
    const EXT: &'static str = "hbs";

    fn init(templates: &[(&str, &TemplateInfo)]) -> Option<Handlebars> {
        let mut hb = Handlebars::new();
        for &(name, info) in templates {
            let path = &info.path;
            if let Err(e) = hb.register_template_file(name, path) {
                warn!("Error in Handlebars template '{}'.", name);
                info_!("{}", e);
                info_!("Template path: '{}'.", path.to_string_lossy());
                return None;
            }
        }

        Some(hb)
    }

    fn render<C: Serialize>(&self, name: &str, context: C) -> Option<String> {
        if self.get_template(name).is_none() {
            warn!("Handlebars template '{}' does not exist.", name);
            return None;
        }

        match Handlebars::render(self, name, &context) {
            Ok(string) => Some(string),
            Err(e) => {
                warn!("Error rendering Handlebars template '{}': {}", name, e);
                None
            }
        }
    }
}
