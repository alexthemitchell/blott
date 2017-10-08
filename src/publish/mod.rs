extern crate s3;

mod private_strings;

use self::s3::bucket::Bucket;
use self::s3::credentials::Credentials;

use self::private_strings::{ACCESS_KEY_ID, SECRET_ACCESS_KEY, BUCKET_NAME, BASE_URL, REGION};

use std::result;

type URL = String;
type Result<T> = result::Result<T, PublishError>;

#[derive(Debug)]
pub enum PublishError {
    S3(s3::error::S3Error),
}

fn get_credentials() -> Credentials {
    Credentials::new(ACCESS_KEY_ID, SECRET_ACCESS_KEY, None)
}

fn fetch_bucket() -> Bucket {
    Bucket::new(BUCKET_NAME, REGION, get_credentials())
}

fn url_from_filename(name: &str) -> URL {
    BASE_URL.to_owned() + name
}

fn publish_to_bucket(bucket: Bucket, name: &str, contents: &str) -> Result<URL> {
    match bucket.put(name, contents.as_bytes(), "text/html") {
        Ok(_) => Ok(url_from_filename(name)),
        Err(e) => Err(PublishError::S3(e)),
    }
}

pub fn publish_file(name: &str, contents: &str) -> Result<URL> {
    publish_to_bucket(fetch_bucket(), name, contents)
}
