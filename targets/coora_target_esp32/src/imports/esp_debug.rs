use coora_engine::Debug;

pub struct EspDebug;

impl Debug for EspDebug {
    fn log(&mut self, val: &str) {
        println!("{val}");
        //TODO post
    }
}
