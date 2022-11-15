use super::WifiClient;
use crate::{wifi, Store};
use anyhow::Result;
use embedded_svc::ipv4;
use embedded_svc::{
    ipv4::{Mask, Subnet},
    wifi::{AccessPointConfiguration, ApIpStatus, ApStatus, Configuration, Wifi},
};
use esp_idf_svc::wifi::EspWifi;
use std::net::Ipv4Addr;

pub struct WifiAccessPoint;
//  {
// pub wifi: EspWifi,
// pub gateway: Ipv4Addr,
// pub settings: AccessPointSettings,
// }

impl WifiAccessPoint {
    pub fn named_from_store(_store: &Store, wifi: &mut EspWifi) -> Result<WifiClient> {
        //TODO device with id
        Self::new(wifi, "cool esp32 🤘")
    }

    pub fn new(wifi: &mut EspWifi, ssid: &str) -> Result<WifiClient> {
        let gateway: Ipv4Addr = Ipv4Addr::new(192, 168, 7, 1);
        let config = ipv4::RouterConfiguration {
            subnet: Subnet {
                gateway,
                mask: Mask(24), //default
            },
            ..Default::default()
        };

        wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
            ssid: ssid.into(),
            // password: password.into(),
            // ssid_hidden: bool,
            // channel: u8,
            // secondary_channel: Option<u8>,
            // protocols: EnumSet<Protocol>,
            // auth_method: AuthMethod,
            // password: heapless::String<64>,
            // max_connections: u16,
            ip_conf: Some(config),
            ..Default::default()
        }))?;

        println!("WIFI AP - creating access point '{}'...", ssid);
        wifi.wait_status_with_timeout(wifi::utility::TIMEOUT_DURATION, |s| !s.is_transitional())
            .map_err(|e| anyhow::anyhow!("WIFI AP - timeout: {:?}", e))?;

        let status = wifi.get_status();

        match status.1 {
            ApStatus::Started(ApIpStatus::Done) => {}
            _ => {}
        }

        if let ApStatus::Started(ApIpStatus::Done) = status.1 {
            println!("\n\nWIFI AP - ready");
            Ok(WifiClient {
                settings: ipv4::ClientSettings {
                    subnet: config.subnet,
                    dns: config.dns,
                    secondary_dns: config.secondary_dns,
                    ip: gateway,
                }, // settings: client_settings,
            })
        } else {
            Err(anyhow::anyhow!("WIFI AP - Failed to connect in time."))
        }
    }
    // pub fn start_server(&self) -> Result<EspHttpServer> {
    //     println!("WIFI AP - server running at {}", self);
    //     start_server()
    // }
}
