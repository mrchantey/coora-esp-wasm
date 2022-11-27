use crate::{wifi::*, Store};
use anyhow::Result;

use esp_idf_svc::{http::server::EspHttpServer, wifi::EspWifi};
use extend::ext;

#[ext]
pub impl EspWifi {
    fn start_server(&self, store: &Store) -> Result<EspHttpServer> {
        start_server(store)
    }
    fn set_log_level(&self) {
        // EspLogger::se
    }
}
