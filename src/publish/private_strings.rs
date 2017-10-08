extern crate s3;

use self::s3::region::Region;

pub static ACCESS_KEY_ID:       &'static str = "";
pub static SECRET_ACCESS_KEY:   &'static str = "";
pub static BUCKET_NAME:         &'static str = "";
pub static BASE_URL:            &'static str = "";
pub const REGION: Region = Region::UsWest2;
