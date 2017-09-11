extern crate regex;

use std;

use self::regex::Regex;
use self::templates::Template;

mod markdown;
mod templates;

pub type Result<T> = std::result::Result<T, RenderError>;
pub type HTML = String;

#[derive(Debug)]
pub enum RenderError {
    Markdown(markdown::MarkdownRenderError),
}

pub fn render_post(input_path: std::path::PathBuf) -> Result<HTML> {
    match markdown::render_file(input_path) {
        Ok(contents) => {
            let mut template = templates::get_template(Template::Post);
            Ok(template.replace("{{ title }}", "A Title!").replace(
                "{{ content }}",
                &contents,
            ))
        }
        Err(err) => Err(RenderError::Markdown(err)),
    }
}

fn regex_for_tag(tag: &str) -> regex::Regex {
    Regex::new(&format!("{}", tag)).unwrap()
}
