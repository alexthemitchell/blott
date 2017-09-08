extern crate base64;
extern crate regex;
extern crate seahash;

use std::hash::Hasher;
use std::mem::transmute;

type Hash = u64;

fn tagify_hash(hash: Hash) -> String {
    let non_tweet_safe_char_regex = regex::Regex::new(r"[+/=]").unwrap();
    let hash_bytes: [u8; 8] = unsafe { transmute(hash) };
    let encoded_hash = base64::encode(&hash_bytes);
    let tweetsafe_hash = non_tweet_safe_char_regex.replace_all(&encoded_hash, "");
    tweetsafe_hash[0..7].to_string().to_uppercase()
}

pub fn hash(contents: &[u8]) -> String {
    let mut hasher = seahash::SeaHasher::new();
    hasher.write(contents);
    tagify_hash(hasher.finish())
}

pub fn hashtag(contents: &[u8]) -> String {
    format!("{}{}", "#", hash(contents))
}
