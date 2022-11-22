use anyhow::Result;
use embedded_svc::storage::{RawStorage, StorageBase};
use std::sync::{Arc, Mutex};
// use embedded_svc::storage as _;
use esp_idf_svc::{nvs::*, nvs_storage::*};
use extend::ext;
// const STORE: Arc<Mutex<Option<EspNvsStorage>>> = Arc::new(Mutex::new(None));
// const STORE: Arc<Mutex<Option<EspNvsStorage>>> = Arc::new(Mutex::new(None));

/// Gets EspDefaultNvs & EspNvsStorage
/// Seems like the only one that throws if dropped and recreated in combination with server

pub type Store = Arc<Mutex<EspNvsStorage>>;
pub type Nvs = Arc<EspDefaultNvs>;

pub fn take_nvs() -> Result<Nvs> {
    Ok(Arc::new(EspDefaultNvs::new()?))
}

pub fn take_store(nvs: &Nvs) -> Result<Store> {
    let storage = EspNvsStorage::new_default(Arc::clone(&nvs), "default", true)?;
    Ok(Arc::new(Mutex::new(storage)))
}

pub fn take_nvs_store() -> Result<(Nvs, Store)> {
    let nvs = take_nvs()?;
    let store = take_store(&nvs)?;
    Ok((nvs, store))
}

// impl NvsStore {
//     pub fn new() -> Result<NvsStore> {
//         let nvs = Arc::new(EspDefaultNvs::new()?);
//         let storage = EspNvsStorage::new_default(Arc::clone(&nvs), "default", true)?;
//         let store = Arc::new(Mutex::new(storage));
//         Ok(NvsStore { nvs, store })
//     }
// }

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
    fn set_u32(&self, key: &str, val: u32) -> Result<()> {
        let mut store = self.lock().unwrap();
        store.put_raw(key, &val.to_le_bytes())?;
        Ok(())
    }

    fn get_u32(&self, key: &str) -> Result<u32> {
        let store = self.lock().unwrap();
        let mut buff = [0; 4];
        if let Some(_) = store.get_raw(key, &mut buff)? {
            Ok(u32::from_le_bytes(buff))
        } else {
            Err(anyhow::anyhow!("key not found: {key}"))
        }
    }

    fn get<const T: usize>(&self, key: &str) -> Result<([u8; T], usize)> {
        let mut buf = [0; T];
        match self.get_with(key, &mut buf) {
            Ok(len) => Ok((buf, len)),
            Err(err) => Err(err),
        }
    }
    fn get_with(&self, key: &str, buff: &mut [u8]) -> Result<usize> {
        let store = self.lock().unwrap();
        if let Some((_, len)) = store.get_raw(key, buff)? {
            Ok(len)
        } else {
            Err(anyhow::anyhow!("key not found: {key}"))
        }
    }
}
