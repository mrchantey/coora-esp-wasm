use anyhow::Result;
use coora_target_esp32::{wifi::get_wifi, *};

fn main() -> Result<()> {
    utility::print_free_heap("nothing"); //288.26
    {
        let (nvs, _) = take_nvs_store()?;
        utility::print_free_heap("with store"); //284.81
        let mut _wifi = get_wifi(&nvs)?;
        utility::print_free_heap("with wifi"); //279
    }
    utility::print_free_heap("nothing"); //279.26
    {
        let (nvs, _) = take_nvs_store()?;
        utility::print_free_heap("with store"); //272
        let mut _wifi = get_wifi(&nvs)?;
        utility::print_free_heap("with wifi"); //244
    }
    utility::print_free_heap("nothing"); //279.26
    Ok(())
}
