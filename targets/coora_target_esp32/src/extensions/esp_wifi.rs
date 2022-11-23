use crate::{wifi::*, Store};
use anyhow::Result;
use embedded_svc::http::Method;
use esp_idf_svc::{http::server::EspHttpServer, wifi::EspWifi};
use extend::ext;

#[ext]
pub impl EspWifi {
    fn get(&self, url: impl AsRef<str>) -> Result<()> {
        fetch(Method::Get, url)
    }
    fn start_server(&self, store: &Store) -> Result<EspHttpServer> {
        start_server(store)
    }
}
