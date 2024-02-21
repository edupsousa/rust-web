use handlebars::{DirectorySourceOptions, Handlebars, TemplateError};

pub type TemplateEngine = Handlebars<'static>;

pub fn build_template_engine() -> Result<TemplateEngine, TemplateError> {
    let mut handlebars = Handlebars::new();
    if cfg!(debug_assertions) {
        handlebars.set_dev_mode(true);
    }
    let options = DirectorySourceOptions::default();

    handlebars.register_templates_directory("templates/", options)?;

    Ok(handlebars)
}
