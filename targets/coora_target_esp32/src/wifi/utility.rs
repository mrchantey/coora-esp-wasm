use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::EspWifi;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;

use crate::NvsStore;

// pub const timeout_secs: u32 = 10;
pub const TIMEOUT_DURATION: Duration = Duration::from_secs(2);

//should be take wifi!
pub fn get_wifi(store: &NvsStore) -> Result<EspWifi> {
    //this is a take as well i think!!!
    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_look_stack = Arc::new(EspSysLoopStack::new()?);
    // let nvs = Arc::new(EspDefaultNvs::new()?);
    let nvs = Arc::clone(&store.nvs);
    let wifi = EspWifi::new(netif_stack, sys_look_stack, nvs)?;
    Ok(wifi)
}
