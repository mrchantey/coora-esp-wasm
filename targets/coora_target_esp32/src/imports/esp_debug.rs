use coora_engine::Debug;
use esp_idf_svc::http::client::EspHttpClient;
use std::sync::{Arc, Mutex};

use crate::EspHttpClientExt;

pub struct EspDebug<'a> {
    url: &'a str,
    http: Arc<Mutex<EspHttpClient>>,
}

impl<'a> EspDebug<'a> {
    pub fn new(url: &'a str, http: Arc<Mutex<EspHttpClient>>) -> EspDebug<'a> {
        EspDebug { url, http }
    }
}

impl<'a> Debug for EspDebug<'a> {
    fn log(&mut self, val: &str) {
        println!("{val}");
        let mut http = self.http.lock().unwrap();
        http.post(self.url, val.as_bytes()).unwrap();
    }
}
