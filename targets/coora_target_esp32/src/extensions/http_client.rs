use anyhow::Result;
use embedded_svc::{
    http::{client::*, Method},
    io::{Read, Write},
};
use esp_idf_svc::http::client::*;
use esp_idf_sys::EspError;
use extend::ext;

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
    fn read_bytes(&mut self) -> usize {
        let mut total_size = 0;
        // 5. if the status is OK, read self data chunk by chunk into a buffer and print it until done
        let mut buf = [0_u8; 256];
        let mut reader = self.reader();
        loop {
            let Ok(size) = Read::read(&mut reader, &mut buf) else { continue };
            if size == 0 {
                break;
            }
            total_size += size;
        }
        // let _response_text = str::from_utf8(&buf[..size])?;
        // println!("{}", response_text);
        total_size
    }
}
