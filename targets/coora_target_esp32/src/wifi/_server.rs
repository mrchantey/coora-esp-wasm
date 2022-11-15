//https://github.com/ferrous-systems/espressif-trainings/blob/main/intro/http-server/src/main.rs
//https://github.com/esp-rs/esp-idf-svc
// use crate::*;
use anyhow::Result;
use core::str;
use embedded_svc::http::server::registry::Registry;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use std::sync::Arc;

use crate::{utility, EspHttpRequest_Ext, EspHttpResponse_Ext, Store};

use super::{WifiCredentials, CREDENTIALS_MAX_LEN};

pub fn start_server(store: &Store) -> Result<EspHttpServer> {
    let server_config = Configuration::default();
    let mut server = EspHttpServer::new(&server_config)?;
    server.handle_get("/", |_request, response| {
        response.write_bytes(templated("welcome").as_bytes())?;
        Ok(())
    })?;

    server.handle_get("/nicha", move |_request, response| {
        response.write_bytes(templated("❤️❤️❤️HELLO FROM NICHA!❤️❤️❤️").as_bytes())?;
        Ok(())
    })?;
    let store1 = Arc::clone(&store);
    server.handle_get("/clear-wifi", move |_request, response| {
        WifiCredentials::clear(&store1)?;
        response.ok()?;
        Ok(())
    })?;
    server.handle_get("/restart", move |_request, response| {
        //TODO signal restart, this will never send
        response.ok()?;
        utility::restart();
    })?;

    let store1 = Arc::clone(&store);
    server.handle_post("/set-ssid", move |mut request, response| {
        let (buf, len) = request.read_bytes::<{ CREDENTIALS_MAX_LEN }>()?;
        WifiCredentials::set_ssid(&store1, &buf[..len])?;
        response.ok()?;
        Ok(())
    })?;
    let store1 = Arc::clone(&store);
    server.handle_post("/set-pass", move |mut request, response| {
        let (buf, len) = request.read_bytes::<{ CREDENTIALS_MAX_LEN }>()?;
        WifiCredentials::set_pass(&store1, &buf[..len])?;
        response.ok()?;
        Ok(())
    })?;
    server.handle_get("/ping", move |_, response| {
        println!("\nping received!");
        response.ok()?;
        Ok(())
    })?;

    Ok(server)
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
		<style>
		hmtl,body{{
			background:black;
			color:white;
		}}
		</style>
        <meta charset="utf-8">
        <title>howdy</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}
