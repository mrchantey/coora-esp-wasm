use anyhow::Result;
use embedded_svc::{
    http::{client::*, Method},
    io::{Read, Write},
};
use esp_idf_svc::http::client::*;
use esp_idf_sys::EspError;
use extend::ext;
use std::str;

#[ext]
pub impl EspHttpClient {
    fn new_https() -> Result<EspHttpClient, EspError> {
        EspHttpClient::new(&EspHttpClientConfiguration {
            use_global_ca_store: true,
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
            ..Default::default()
        })
    }

    fn get(&mut self, url: impl AsRef<str>) -> Result<EspHttpResponse> {
        self.fetch(Method::Get, url, &[])
    }
    fn post(&mut self, url: impl AsRef<str>, body: &[u8]) -> Result<EspHttpResponse> {
        self.fetch(Method::Post, url, body)
    }

    fn fetch(
        &mut self,
        method: Method,
        url: impl AsRef<str>,
        body: &[u8],
    ) -> anyhow::Result<EspHttpResponse> {
        //no https
        // let mut client = EspHttpClient::new_default()?;
        //TODO https behind feature, its really big
        let request = self.request(method, url.as_ref())?;
        let mut writer = request.into_writer(body.len())?;
        writer.write_all(body)?;

        let response = writer.submit()?;

        Ok(response)
    }
}

#[ext(name = ClientHttpResponse)]
pub impl EspHttpResponse<'_> {
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut total_size = 0;
        let mut reader = self.reader();
        loop {
            let size = reader.read(buf)?;
            if size == 0 {
                break;
            }
            total_size += size;
        }
        // let _response_text = str::from_utf8(&buf[..size])?;
        // println!("{}", response_text);
        Ok(total_size)
    }
    fn read_str<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a str> {
        let len = self.read_bytes(buf)?;
        let str = str::from_utf8(&buf[..len])?;
        Ok(str)
    }
}
