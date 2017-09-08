use std::env;
use std::fs;

mod markdown;

fn main() {
    for argument in env::args() {
        let path = fs::canonicalize(argument);
        match path {
            Ok(input_path) => {
                match markdown::render_file(input_path) {
                    Ok(html) => println!("{}", html),
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
