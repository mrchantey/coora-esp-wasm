use super::*;
use crate::{
    utility::b_to_kb,
    wifi::{get_wifi, WifiFallbackClient},
    *,
};

use anyhow::Result;
use embedded_svc::http::server::registry::Registry;
use esp_idf_svc::{http::server::EspHttpServer, wifi::EspWifi};
use std::sync::{Arc, Mutex};

pub struct SketchServer {
    // pub settings: Option<ClientSettings>,
    pub wifi: EspWifi,
    pub store: Store,
    pub sketch_buffer: Arc<Mutex<SketchBuffer>>,
    pub client: WifiFallbackClient,
    pub server: Option<EspHttpServer>,
    pub reload_mode: SketchReloadMode,
}

impl SketchServer {
    pub fn new(
        sketch_buffer: Arc<Mutex<SketchBuffer>>,
        nvs: &Nvs,
        store: &mut Store,
        reload_mode: SketchReloadMode,
    ) -> Result<SketchServer> {
        let mut wifi = get_wifi(nvs)?;
        let store = Arc::clone(store);
        // wifi.set
        let client = WifiFallbackClient::new_from_store(&mut wifi, &store)?;

        Ok(SketchServer {
            wifi,
            store,
            client,
            reload_mode,
            sketch_buffer,
            server: None,
            // settings: None,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        if self.client.connected {
            Ok(())
        } else if let Some(_settings) = self.client.check_status(&mut self.wifi)? {
            // self.settings = Some(settings);
            self.start_server()
        } else {
            Ok(())
        }
    }

    pub fn start_server(&mut self) -> Result<()> {
        let mut server = self.wifi.start_server(&self.store)?;
        self.set_handle_sketch(&mut server, "/sketch", self.reload_mode.clone())?;
        self.set_handle_sketch(&mut server, "/sketch-nvs", SketchReloadMode::RestartDevice)?;
        self.server = Some(server);
        Ok(())
    }

    fn set_handle_sketch(
        &mut self,
        server: &mut EspHttpServer,
        name: &str,
        reload_mode: SketchReloadMode,
    ) -> Result<()> {
        let store = self.store.clone();
        let buffer = self.sketch_buffer.clone();
        server.handle_post(name, move |mut request, response| {
            let mut buffer = buffer.lock().unwrap();
            buffer.from_request(&mut request)?;
            //TODO optionally dont nvs and attempt no restart
            if reload_mode == SketchReloadMode::RestartDevice {
                buffer.set_nvs(&store)?;
                //TODO return OK then restart device
            }
            println!("\nSKETCH received! {}", b_to_kb(buffer.len));
            response.ok()?;
            Ok(())
        })?;
        Ok(())
    }
}
