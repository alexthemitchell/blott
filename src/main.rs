use std::env;
use std::fs;

mod hash;
mod markdown;

fn main() {
    for argument in env::args() {
        let path = fs::canonicalize(argument);
        match path {
            Ok(input_path) => {
                match markdown::render_file(input_path) {
                    Ok(html) => {
                        println!(
                            "OutputHTML: {}\n\nHash: {}",
                            html,
                            hash::hashtag(html.as_bytes())
                        )
                    }
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(_) => {
                println!("There was an error ;(");
                return;
            }
        }
    }
}
