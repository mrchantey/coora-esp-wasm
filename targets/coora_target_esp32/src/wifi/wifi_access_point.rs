use crate::wifi;
use anyhow::{Ok, Result};
use embedded_svc::ipv4;
use embedded_svc::{
    ipv4::{Mask, Subnet},
    wifi::{AccessPointConfiguration, ApIpStatus, ApStatus, Configuration, Wifi},
};
use esp_idf_svc::wifi::EspWifi;
use std::net::Ipv4Addr;

pub struct WifiAccessPoint {
    config: AccessPointConfiguration,
}

impl WifiAccessPoint {
    // pub fn named_from_store(_store: &Store, wifi: &mut EspWifi) -> Result<WifiClient> {
    //     //TODO device with id
    // }
    pub fn new_default(wifi: &mut EspWifi) -> Result<WifiAccessPoint> {
        Self::new(wifi, "cool esp32 🤘")
    }

    pub fn new(wifi: &mut EspWifi, ssid: &str) -> Result<WifiAccessPoint> {
        let gateway: Ipv4Addr = Ipv4Addr::new(192, 168, 7, 1);
        let ip_conf = ipv4::RouterConfiguration {
            subnet: Subnet {
                gateway,
                mask: Mask(24), //default
            },
            ..Default::default()
        };

        let config = AccessPointConfiguration {
            ssid: ssid.into(),
            // password: password.into(),
            // ssid_hidden: bool,
            // channel: u8,
            // secondary_channel: Option<u8>,
            // protocols: EnumSet<Protocol>,
            // auth_method: AuthMethod,
            // password: heapless::String<64>,
            // max_connections: u16,
            ip_conf: Some(ip_conf),
            ..Default::default()
        };

        wifi.set_configuration(&Configuration::AccessPoint(config))?;

        Ok(WifiAccessPoint { config })
    }

    //todo throw on timeout
    pub fn check_status(&self, wifi: &EspWifi) -> Option<ipv4::ClientSettings> {
        let status = wifi.get_status();
        if let ApStatus::Started(ApIpStatus::Done) = status.1 {
            println!("\n\nWIFI AP - ready, ssid: {}", self.config.ssid);
            let ip_conf = self.config.ip_conf.unwrap();
            Some(ipv4::ClientSettings {
                subnet: ip_conf.subnet,
                dns: ip_conf.dns,
                secondary_dns: ip_conf.secondary_dns,
                ip: ip_conf.subnet.gateway,
            })
        } else {
            None
        }
    }

    pub fn check_status_sync(&self, wifi: &EspWifi) -> Result<ipv4::ClientSettings> {
        println!("WIFI AP - creating access point '{}'...", self.config.ssid);
        wifi.wait_status_with_timeout(wifi::TIMEOUT_DURATION, |s| !s.is_transitional())
            .map_err(|e| anyhow::anyhow!("WIFI AP - timeout: {:?}", e))?;
        if let Some(client_settings) = self.check_status(wifi) {
            Ok(client_settings)
        } else {
            Err(anyhow::anyhow!("WIFI AP - Failed to connect in time."))
        }
    }
}
