use anyhow::Result;
use embedded_svc::ipv4::ClientSettings;
use embedded_svc::wifi::{
    ClientConfiguration, ClientConnectionStatus, ClientIpStatus, ClientStatus, Configuration, Wifi,
};
use esp_idf_svc::wifi::EspWifi;
use std::str;

use crate::{wifi, Store};

use super::WifiCredentials;
pub struct WifiClient {
    config: ClientConfiguration,
}

impl WifiClient {
    pub fn from_store(wifi: &mut EspWifi, store: &Store) -> Result<WifiClient> {
        let WifiCredentials { ssid, pass } = WifiCredentials::get(store)?;
        Self::new(wifi, ssid.as_str(), pass.as_str())
    }

    pub fn new(wifi: &mut EspWifi, ssid: &str, password: &str) -> Result<WifiClient> {
        let config = ClientConfiguration {
            ssid: ssid.into(),
            password: password.into(),
            ..Default::default()
        };
        wifi.set_configuration(&Configuration::Client(config.clone()))?;

        println!("WIFI CLIENT - connecting to {}...", ssid);
        Ok(WifiClient { config })
    }

    pub fn check_status(&self, wifi: &EspWifi) -> Option<ClientSettings> {
        let status = wifi.get_status();
        if let ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
            client_settings,
        ))) = status.0
        {
            println!(
                "\n\nWIFI CLIENT - connected\nWIFI CLIENT - IP: {:?}\n\n",
                client_settings.ip
            );
            Some(client_settings)
        } else {
            None
        }
    }

    pub fn check_status_sync(&self, wifi: &EspWifi) -> Result<ClientSettings> {
        wifi.wait_status_with_timeout(wifi::TIMEOUT_DURATION, |s| !s.is_transitional())
            .map_err(|e| anyhow::anyhow!("WIFI CLIENT - timeout: {:?}", e))?;

        if let Some(settings) = self.check_status(wifi) {
            Ok(settings)
        } else {
            Err(anyhow::anyhow!("WIFI CLIENT - failed to connect in time."))
        }
    }
}
