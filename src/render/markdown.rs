use std;
use std::fs;
use std::io::Read;

extern crate markdown;

pub type Result<T> = std::result::Result<T, MarkdownRenderError>;

#[derive(Debug)]
pub enum MarkdownRenderError {
    FileOpen,
    FileRead,
    UTF8Parse,
}

fn render_string(markdown_string: String) -> String {
    markdown::to_html(&markdown_string)
}

fn render_vec(markdown_bytes: Vec<u8>) -> Result<String> {
    match String::from_utf8(markdown_bytes) {
        Ok(markdown_string) => Ok(render_string(markdown_string)),
        Err(_) => Err(MarkdownRenderError::UTF8Parse),
    }
}

pub fn render_file(path: std::path::PathBuf) -> Result<String> {
    let mut contents = Vec::new();
    match fs::File::open(path) {
        Ok(mut file) => {
            match file.read_to_end(&mut contents) {
                Ok(_) => render_vec(contents),
                Err(_) => Err(MarkdownRenderError::FileRead),
            }
        }
        Err(_) => Err(MarkdownRenderError::FileOpen),
    }
}
