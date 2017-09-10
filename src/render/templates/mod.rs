use std::fs::{self,File};
use std::io::{self,Read};
use std::path::PathBuf;

mod post;

pub enum Template {
    Post,
}

pub fn get_template(template: Template) -> String {
    match template {
        Post => post::Template.to_string(),
    }
}

