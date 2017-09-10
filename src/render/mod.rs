use std;

mod markdown;
mod templates;

use self::templates::Template;

pub type Result<T> = std::result::Result<T, RenderError>;
pub type HTML = String;

#[derive(Debug)]
pub enum RenderError {
    Markdown(markdown::MarkdownRenderError),
}

pub fn render_file(path: std::path::PathBuf) -> Result<HTML> {
    let template = templates::get_template(Template::Post);
    Ok(template)
}

