use anyhow::Result;
use embedded_svc::ipv4;
use esp_idf_svc::wifi::EspWifi;

use crate::{
    utility::{sleep_ms, Timeout},
    wifi, Store,
};

use super::*;

pub struct WifiFallbackClient {
    pub timeout: Timeout,
    pub client: Option<WifiClient>,
    pub connected: bool,
    pub access_point: Option<WifiAccessPoint>,
}

impl WifiFallbackClient {
    pub fn new_from_store(wifi: &mut EspWifi, store: &Store) -> Result<WifiFallbackClient> {
        let mut client = None;
        let mut access_point = None;
        if let Ok(wifi_client) = WifiClient::from_store(wifi, store) {
            client = Some(wifi_client);
        } else {
            println!("_\nWIFI CLIENT - no credentials found, falling back to ap..\n_");
            access_point = Some(WifiAccessPoint::new_default(wifi)?);
        }

        Ok(WifiFallbackClient {
            timeout: Timeout::new_wifi(),
            client,
            connected: false,
            access_point,
        })
    }

    pub fn new(wifi: &mut EspWifi, ssid: &str, pass: &str) -> Result<WifiFallbackClient> {
        Ok(WifiFallbackClient {
            timeout: Timeout::new_wifi(),
            client: Some(WifiClient::new(wifi, ssid, pass)?),
            connected: false,
            access_point: None,
        })
    }
    ///check status, mutable because may switch to AP fallback after timeout
    pub fn check_status(&mut self, wifi: &mut EspWifi) -> Result<Option<ipv4::ClientSettings>> {
        if let Some(client) = &self.client {
            match client.check_status(wifi) {
                Ok(status) => {
                    if let Some(status) = status {
                        self.connected = true;
                        Ok(Some(status))
                    } else {
                        Ok(None)
                    }
                }
                _ => {
									self.switch_to_ap(wifi)?;
									self.check_status(wifi)
								}
            }
        } else if let Some(ap) = &self.access_point {
            //TODO ap timeout?
            if let Some(status) = ap.check_status(wifi)? {
                self.connected = true;
                Ok(Some(status))
            } else {
                Ok(None)
            }
        } else {
            Err(anyhow::anyhow!("we should never have neither client or ap"))
        }
    }

		#[rustfmt::skip]
    pub fn check_status_sync(&mut self,wifi:&mut EspWifi)->Result<ipv4::ClientSettings>{
			loop{
				if let Some(settings) = self.check_status(wifi)?{
					break Ok(settings)
				}
				sleep_ms(wifi::STATUS_POLL_MILLIS);
			}
		}

    fn switch_to_ap(&mut self, wifi: &mut EspWifi) -> Result<()> {
        self.client = None;
        self.timeout.reset();
        self.access_point = Some(WifiAccessPoint::new_default(wifi)?);
        Ok(())
    }
}
