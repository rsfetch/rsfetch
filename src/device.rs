use std::fs;
use crate::*;

pub struct DeviceInfo {
    model: String,
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        DeviceInfo { model: String::new(), }
    }

    pub fn get(&mut self) -> Result<()> {
        let mut path = "/sys/devices/virtual/dmi/id/product_name";
        if !fs::metadata(path).is_ok() {
            path = "/sys/firmware/devicetree/base/model";
        }

        let f = fs::read_to_string(path)
            .context(DeviceName)?;
        self.model = f.trim().to_string();

        Ok(())
    }

    // format it
    pub fn format(&self) -> String { self.model.clone() }
}
