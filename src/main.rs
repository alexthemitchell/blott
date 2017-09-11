#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

use std::env;
use std::fs;
use std::path::PathBuf;

mod hash;
mod render;
mod tweet;

fn main() {
    let matches = clap_app!
        (blott =>
         (version: "1.0")
         (author: "Alex Mitchell <alex@alexthemitchell.com")
         (about: "Static blog platform")
         (@arg INPUT: +required +takes_value -i --input "Markdown file to use as input")
        ).get_matches();

    let file = matches.value_of("INPUT");
    let mut path = PathBuf::new();
    path.push(file.unwrap());
    match render::render_post(path) {
        Ok(html) => {
            let hashtag = hash::hashtag(html.clone());
            println!("Hashtag: {}\nHTML:\n{}", hashtag, html);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
