use std::time::Duration;

use anyhow::Result;
use embedded_svc::mqtt::client::*;
use esp_idf_svc::mqtt::client::*;
pub struct MqttClient {
    pub client: EspMqttClient,
}

const BUFFER_SIZE: usize = 1024;

impl MqttClient {
    pub fn new() -> Result<MqttClient> {
        let config = MqttClientConfiguration {
            // protocol_version: Option<MqttProtocolVersion>,
            client_id: Some("esp32-client"),
            connection_refresh_interval: Duration::from_secs(15),
            keep_alive_interval: Some(Duration::from_secs(1)),
            reconnect_timeout: Some(Duration::from_secs(2)),
            network_timeout: Duration::from_secs(10),
            lwt: Some(LwtConfiguration {
                topic: "imdead/1",
                payload: b"its been fun",
                qos: QoS::AtLeastOnce,
                retain: false,
            }),
            // disable_clean_session: bool,
            // task_prio: u8,
            // task_stack: usize,
            buffer_size: BUFFER_SIZE,
            out_buffer_size: BUFFER_SIZE,

            username: Some("esp32-me"),
            password: Some("esp32-me"),

            // use_global_ca_store: bool,
            // skip_cert_common_name_check: bool,
            // crt_bundle_attach: Option<unsafe extern "C" fn(conf: *mut c_types::c_void) -> esp_err_t>,
            ..Default::default()
        };
        //secure: ssl://127.0.0.1:8883
        //insecure: tcp://127.0.0.1:1883
        //insecure:
        let url = "mqtt://127.0.0.1:1883";
        let client = EspMqttClient::new(url, &config, |result| match result {
            Ok(msg) => println!("message received: {:?}", msg),
            Err(err) => println!("something went wrong: {}", err),
        })?;

        Ok(MqttClient { client })
    }

    pub fn publish(&mut self, payload: &[u8]) -> Result<MessageId> {
        let id = self
            .client
            .publish("testtopic/9", QoS::AtLeastOnce, false, payload)?;
        Ok(id)
    }

    pub fn subscribe(&mut self) -> Result<MessageId> {
        let id = self.client.subscribe("testtopic/9", QoS::AtLeastOnce)?;
        Ok(id)
    }
}
