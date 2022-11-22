use anyhow::Result;
use embedded_svc::http::Method;
use embedded_svc::ipv4::ClientSettings;
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::wifi::EspWifi;
use std::str;

use crate::wifi::EspWifiExt;
use crate::{wifi, Store};

use super::{fetch, start_server, WifiAccessPoint, WifiCredentials};
pub struct WifiClient {
    // pub wifi: EspWifi,
    pub settings: ClientSettings,
}

impl WifiClient {
    pub fn from_store_or_ap(store: &Store, wifi: &mut EspWifi) -> Result<WifiClient> {
        match Self::from_store(store, wifi) {
            Ok(client) => Ok(client),
            Err(_err) => {
                println!("_\n{}\nWIFI CLIENT - falling back to ap..\n_", _err);
                let client = WifiAccessPoint::named_from_store(store, wifi)?;
                Ok(client)
            }
        }
    }

    pub fn from_store(store: &Store, wifi: &mut EspWifi) -> Result<WifiClient> {
        let credentials = WifiCredentials::get(store)?;
        let ssid = str::from_utf8(&credentials.ssid[..credentials.ssid_len])?;
        let pass = str::from_utf8(&credentials.pass[..credentials.pass_len])?;
        Self::new(wifi, ssid, pass)
    }

    pub fn new(wifi: &mut EspWifi, ssid: &str, password: &str) -> Result<WifiClient> {
        wifi.set_config(ssid, password)?;
        println!("WIFI CLIENT - connecting to {}...", ssid);
        wifi.wait_status_with_timeout(wifi::TIMEOUT_DURATION, |s| !s.is_transitional())
            .map_err(|e| anyhow::anyhow!("WIFI CLIENT - timeout: {:?}", e))?;

        if let Some(settings) = wifi.check_status() {
            Ok(WifiClient { settings })
        } else {
            Err(anyhow::anyhow!("WIFI CLIENT - failed to connect in time."))
        }
    }

    pub fn get(&self, url: impl AsRef<str>) -> Result<()> {
        fetch(Method::Get, url)
    }

    pub fn start_server(&self, store: &Store) -> Result<EspHttpServer> {
        println!(
            "WIFI CLIENT - server running at http://{:?}",
            self.settings.ip
        );
        start_server(store)
    }

    // pub fn post(url: impl AsRef<str>) -> Result<()> {

    // }
}
