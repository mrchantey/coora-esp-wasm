use crate::{
    utility::{self, b_to_kb},
    ArcMutexEspNvsStorageExt, EspHttpRequest_Ext, Store,
};
use anyhow::Result;
use coora_engine::WasmApp;
use esp_idf_svc::http::server::EspHttpRequest;

pub const MAX_SKETCH_SIZE: usize = 8 * 1024;

pub const STORE_SKETCH_BUF: &str = "sketch_buf";

pub struct SketchBuffer {
    pub dirty: bool,
    pub buffer: Box<[u8; MAX_SKETCH_SIZE]>,
    pub len: usize,
}

impl SketchBuffer {
    pub fn new() -> Self {
        SketchBuffer {
            dirty: false,
            buffer: Box::new([0; MAX_SKETCH_SIZE]),
            len: 0,
        }
    }
    pub fn from_nvs_or_default(store: &Store) -> Self {
        let mut buf = Self::new();
        if !utility::was_ok_reset() {
            println!("SKETCH - bad reset, loading default sketch..");
        } else if let Ok(len) = store.get_with(STORE_SKETCH_BUF, &mut *buf.buffer) {
            buf.len = len;
            buf.dirty = true;
            println!("SKETCH - loading nvs sketch.. {}", b_to_kb(buf.len));
            return buf;
        } else {
            println!("SKETCH - loading default sketch..");
        }
        buf.set_bytes(WasmApp::default_wasm());
        buf
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = Self::new();
        buf.set_bytes(bytes);
        buf
    }
    pub fn set_nvs(&mut self, store: &Store) -> Result<()> {
        store.set(STORE_SKETCH_BUF, &self.buffer[..self.len])
    }

    pub fn zero_out(&mut self) {
        for el in self.buffer.iter_mut() {
            *el = 0
        }
    }

    pub fn set_bytes(&mut self, bytes: &[u8]) {
        // self.zero_out();
        self.len = bytes.len();
        self.buffer[..self.len].clone_from_slice(bytes);
        self.dirty = true;
    }

    pub fn from_request(&mut self, request: &mut EspHttpRequest) -> Result<()> {
        // self.zero_out();
        self.len = request.read_bytes_to_arr(&mut self.buffer)?;
        self.dirty = true;
        Ok(())
    }
}
