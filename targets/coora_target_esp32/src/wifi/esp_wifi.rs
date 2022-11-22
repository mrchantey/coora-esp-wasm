use anyhow::Result;
use embedded_svc::{
    http::Method,
    ipv4::ClientSettings,
    wifi::{
        ClientConfiguration, ClientConnectionStatus, ClientIpStatus, ClientStatus, Configuration,
        Wifi,
    },
};
use esp_idf_svc::wifi::EspWifi;
use extend::ext;

use super::fetch;
#[ext]
pub impl EspWifi {
    fn set_config(&mut self, ssid: &str, password: &str) -> Result<()> {
        self.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: ssid.into(),
            password: password.into(),
            ..Default::default()
        }))?;
        Ok(())
    }
    fn check_status(&self) -> Option<ClientSettings> {
        let status = self.get_status();
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
    fn get(&self, url: impl AsRef<str>) -> Result<()> {
        fetch(Method::Get, url)
    }
}
