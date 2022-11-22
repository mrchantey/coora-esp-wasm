use anyhow::Result;
use embedded_svc::{ http::Method};
use esp_idf_svc::{http::server::EspHttpServer, wifi::EspWifi};
use extend::ext;

use crate::Store;

use super::{fetch, start_server};
#[ext]
pub impl EspWifi {
    fn get(&self, url: impl AsRef<str>) -> Result<()> {
        fetch(Method::Get, url)
    }
    fn start_server(&self, store: &Store) -> Result<EspHttpServer> {
        start_server(store)
    }
}
