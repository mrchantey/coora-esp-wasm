//https://github.com/ferrous-systems/espressif-trainings/blob/main/intro/http-server/src/main.rs
//https://github.com/esp-rs/esp-idf-svc
use anyhow::Result;
use core::str;
use std::{
	sync::{Arc, Mutex},
	thread::sleep,
	time::Duration,
};

use embedded_svc::{
	http::server::{registry::Registry, Request, Response},
	io::{Read, Write},
};
use esp_idf_svc::http::server::{Configuration, EspHttpRequest, EspHttpServer};

use crate::{EspHttpRequest_Ext, EspHttpResponse_Ext};

pub fn start_server() -> Result<EspHttpServer> {
	let server_config = Configuration::default();
	let mut server = EspHttpServer::new(&server_config)?;
	server.handle_get("/", |_request, response| {
		response.write_bytes(templated("welcome").as_bytes())?;
		Ok(())
	})?;

	server.handle_get("/nicha", move |_request, response| {
		response
			.write_bytes(templated("❤️❤️❤️HELLO FROM NICHA!❤️❤️❤️").as_bytes())?;
		Ok(())
	})?;
	server.handle_post("/ping", move |mut request, response| {
		let (buf, len) = request.read_bytes::<1024>()?;
		println!("\nping received!");
		println!("bytes: {:?}", &buf[0..len]);
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
