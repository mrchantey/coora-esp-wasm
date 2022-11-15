use anyhow::{Ok, Result};
use embedded_svc::storage::{RawStorage, StorageBase};
use std::sync::{Arc, Mutex};
// use embedded_svc::storage as _;
use esp_idf_svc::{nvs::*, nvs_storage::*};
use extend::ext;
// const STORE: Arc<Mutex<Option<EspNvsStorage>>> = Arc::new(Mutex::new(None));
// const STORE: Arc<Mutex<Option<EspNvsStorage>>> = Arc::new(Mutex::new(None));

pub struct NvsStore {
    pub nvs: Arc<EspDefaultNvs>,
    pub store: Store,
}

pub type Store = Arc<Mutex<EspNvsStorage>>;

pub struct StoreBuilder;
#[ext]
pub impl Arc<Mutex<EspNvsStorage>> {
    fn has(&self, key: &str) -> Result<bool> {
        let store = self.lock().unwrap();
        Ok(store.contains(key)?)
    }
    fn remove(&self, key: &str) -> Result<()> {
        let mut store = self.lock().unwrap();
        store.remove(key)?;
        Ok(())
    }
    fn set(&self, key: &str, buf: &[u8]) -> Result<()> {
        let mut store = self.lock().unwrap();
        store.put_raw(key, buf)?;
        Ok(())
    }

    fn get<const T: usize>(&self, key: &str) -> Result<([u8; T], usize)> {
        let store = self.lock().unwrap();
        // .unwrap_or_else(|_| Err(anyhow::anyhow!("key not found: {key}")));
        let mut buff = [0; T];

        if let Some((_, len)) = store.get_raw(key, &mut buff)? {
            Ok((buff, len))
        } else {
            Err(anyhow::anyhow!("key not found: {key}"))
        }
    }
}

impl StoreBuilder {
    pub fn take() -> Result<NvsStore> {
        let nvs = Arc::new(EspDefaultNvs::new()?);
        let storage = EspNvsStorage::new_default(Arc::clone(&nvs), "default", true)?;
        // Ok(Arc::new(Mutex::new(StoreData { nvs, storage })))
        let store = Arc::new(Mutex::new(storage));
        Ok(NvsStore { nvs, store })
    }
}
