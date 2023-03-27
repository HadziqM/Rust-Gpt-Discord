pub mod chat;
pub mod image;

use reqwest::header::HeaderMap;
use super::MyErr;

pub struct Gpt{
    pub(super) head:HeaderMap
}

impl From<reqwest::header::InvalidHeaderValue> for MyErr{
    fn from(value: reqwest::header::InvalidHeaderValue) -> Self {
        MyErr::Custom(format!("reqwest error on invalid header value :{value:?}"))
    }
}
impl Gpt {
    pub fn new()->Result<Self,MyErr>{
        let mut head = HeaderMap::new();
        head.insert(reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"));
        head.insert(reqwest::header::AUTHORIZATION, 
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}",&crate::INIT.gpt.token))?);
        Ok(Gpt { head })
    }
}
