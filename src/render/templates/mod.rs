use std::fs::File;
use std::io::{self,Read};
use std::path::PathBuf;

static POST_TEMPLATE_PATH: &'static str = "post.html";

pub enum Template {
    Post,
}

fn path_for_template(template: Template) -> PathBuf {
    let mut path = PathBuf::new();
    match template {
        Post  => path.push(POST_TEMPLATE_PATH),
    }
    path
}

pub fn get_template(template: Template) -> String {
    let template_path = path_for_template(template);
    let mut template_file = File::open(template_path).unwrap();
    let mut template = String::new();
    template_file.read_to_string(&mut template);
    template
}

#[test]
fn post_template_exists() {
    assert!(fs::canonicalize(POST_TEMPLATE)).is_ok();
}
