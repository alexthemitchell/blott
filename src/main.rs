#[macro_use]
extern crate clap;
extern crate time;

use std::any::Any;
use std::path::PathBuf;

mod render;
mod tweet;
mod publish;

static DATE_FORMAT: &'static str = "%A, %B %e, %Y %H:%M";

fn main() {
    let matches = clap_app!
        (blott =>
         (version: "1.0")
         (author: "Alex Mitchell <alex@alexthemitchell.com")
         (about: "Static blog platform")
         (@arg INPUT: +required +takes_value -i --input "Markdown file to use as input")
         (@arg TITLE: +takes_value -t --title "The post's title")
         (@arg NOTWEET: --notweet "Prints tweet to STDOUT instead of twitter")
         (@arg STDOUT: --stdout "Prints rendered HTML to stdout instead of an output file")
         (@arg OUTPUT: +takes_value -o --output "HTML file to output")
        ).get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let title = matches
        .value_of("TITLE")
        .unwrap_or(&default_title())
        .to_string();
    let output = matches
        .value_of("OUTPUT")
        .unwrap_or(&default_output_filename(title.clone()))
        .to_string();

    let mut path = PathBuf::new();
    path.push(input);

    match render::render_post(title.clone(), path) {
        Ok((html, hashtag)) => {
            match matches.occurrences_of("STDOUT") {
                1 => println!("Hashtag: #{}\nOutput:\n{}", hashtag, html),
                _ => {
                    match publish::publish_file(&output, &html) {
                        Ok(url) => {
                            let tweet_text =
                                format!("\"{}\" by @alexthemitchell. {} #{}", title, url, hashtag);
                            match matches.occurrences_of("NOTWEET") {
                                1 => println!("Not Tweeted: {}", tweet_text),
                                _ => {
                                    match tweet::tweet(&tweet_text) {
                                        Ok(_) => println!("Tweeted: {}", tweet_text),
                                        Err(e) => handle_error(&e),
                                    }
                                }
                            }
                        }
                        Err(e) => handle_error(&e),
                    }
                }
            }
        }
        Err(e) => handle_error(&e),
    }
}

fn default_title() -> String {
    let now = time::now();
    let time_string = now.strftime(DATE_FORMAT).unwrap();
    format!("{}", time_string)
}

fn default_output_filename(title: String) -> String {
    format!("{}.html", &title)
}

fn handle_error(error: &Any) {
    println!("Error: {:?}", error)
}
