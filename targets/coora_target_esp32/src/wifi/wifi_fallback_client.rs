use anyhow::Result;
use embedded_svc::ipv4;
use esp_idf_svc::wifi::EspWifi;
use std::time::SystemTime;

use crate::Store;

use super::*;

pub struct WifiFallbackClient {
    start: SystemTime,
    client: Option<WifiClient>,
    access_point: Option<WifiAccessPoint>,
}

impl WifiFallbackClient {
    pub fn new(wifi: &mut EspWifi, store: &Store) -> Result<WifiFallbackClient> {
        let mut client = None;
        let mut access_point = None;
        if let Ok(wifi_client) = WifiClient::from_store(wifi, store) {
            client = Some(wifi_client);
        } else {
            println!("_\nWIFI CLIENT - no credentials found, falling back to ap..\n_");
            access_point = Some(WifiAccessPoint::new_default(wifi)?);
        }

        Ok(WifiFallbackClient {
            start: SystemTime::now(),
            client,
            access_point,
        })
    }
    ///check status, mutable because may switch to AP fallback after timeout
    pub fn check_status(&mut self, wifi: &mut EspWifi) -> Result<Option<ipv4::ClientSettings>> {
        if let Some(client) = &self.client {
            if let Some(status) = client.check_status(wifi) {
                return Ok(Some(status));
            } else {
                //check for timeout
                let elapsed = SystemTime::now().duration_since(self.start)?;
                if elapsed.ge(&TIMEOUT_DURATION) {
                    println!("_\nWIFI CLIENT - timed out, falling back to ap..\n_");
                    self.switch_to_ap(wifi);
                }
                Ok(None)
            }
        } else if let Some(ap) = &self.access_point {
            Ok(ap.check_status(wifi))
        } else {
            Err(anyhow::anyhow!("we should never have neither client or ap"))
        }
    }

    fn switch_to_ap(&mut self, wifi: &mut EspWifi) -> Result<()> {
        self.client = None;
        self.access_point = Some(WifiAccessPoint::new_default(wifi)?);
        Ok(())
    }
}
