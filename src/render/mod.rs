extern crate regex;

use std;

use self::templates::Template;

mod hash;
mod markdown;
mod templates;

pub type Result<T> = std::result::Result<T, RenderError>;
pub type HTML = String;
pub type Hashtag = String;

#[derive(Debug)]
pub enum RenderError {
    Markdown(markdown::MarkdownRenderError),
}

pub fn render_post(title: String, input_path: std::path::PathBuf) -> Result<(HTML,Hashtag)> {
    match markdown::render_file(input_path) {
        Ok(contents) => {
            let hashtag = hash::hashtag(contents.clone());
            let template = templates::get_template(Template::Post);
            let html = template.replace("{{ title }}", &title)
               .replace("{{ content }}",&contents)
               .replace("{{ hashtag }}", &hashtag);
            Ok((html, hashtag))
        }
        Err(err) => Err(RenderError::Markdown(err)),
    }
}

