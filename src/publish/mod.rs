extern crate s3;

mod private_strings;

use self::s3::bucket::Bucket;
use self::s3::credentials::Credentials;
use self::private_strings::{ACCESS_KEY_ID, SECRET_ACCESS_KEY, REGION, BUCKET_NAME};
use std;

type URL = String;
type Result<T> = std::result::Result<T, PublishError>;

pub enum PublishError {
    S3(s3::error::S3Error),
    URL(std::string::FromUtf8Error),
}

fn load_credentials() -> Credentials {
    Credentials::new(&ACCESS_KEY_ID, &SECRET_ACCESS_KEY, None)
}

pub fn publish_file(name: &str, contents: String) -> Result<URL> {
    let credentials = load_credentials();
    let bucket = Bucket::new(BUCKET_NAME, REGION, credentials);
    match bucket.put(name, contents.as_bytes(), "text/plain") {
        Ok((url, code)) => {
            match String::from_utf8(url) {
                Ok(str_url) => Ok(str_url),
                Err(e) => Err(PublishError::URL(e)),
            }
        }
        Err(e) => Err(PublishError::S3(e)),
    }
}
