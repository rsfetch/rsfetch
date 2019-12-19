use std::fs;
use std::result::Result;

pub struct DeviceInfo {
    model: String,
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        DeviceInfo { model: String::new(), }
    }

    pub fn get(&mut self) -> Result<(), std::io::Error> {
        let f = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")?;
        self.model = f.trim().to_string();

        Ok(())
    }

    // format it
    pub fn format(&self) -> String { self.model.clone() }
}
