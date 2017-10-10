extern crate egg_mode;

mod private_strings;

use self::egg_mode::{tweet, error};

use std::result;

type Result<T> = result::Result<T, TweetError>;

#[derive(Debug)]
pub enum TweetError {
    Send(error::Error),
}

fn get_token() -> egg_mode::Token<'static> {
    let con_token = egg_mode::KeyPair::new(
        private_strings::TWITTER_CONSUMER_KEY,
        private_strings::TWITTER_CONSUMER_SECRET,
    );
    let access_token = egg_mode::KeyPair::new(
        private_strings::TWITTER_ACCESS_TOKEN,
        private_strings::TWITTER_ACCESS_TOKEN_SECRET,
    );
    egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    }
}

pub fn tweet(message: &str) -> Result<egg_mode::Response<tweet::Tweet>> {
    let draft = tweet::DraftTweet::new(message);
    match draft.send(&get_token()) {
        Ok(response) => Ok(response),
        Err(e) => Err(TweetError::Send(e)),
    }
}
