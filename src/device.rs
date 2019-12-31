use std::fs;
use crate::*;
use std::process::Command;

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

        let f = fs::read_to_string(path);
        match f {
            Ok(c)  => self.model = c.trim().to_string(),
            Err(_) => {
                // fallback to sysctl...
                let mut model = String::new();
                let _ = Command::new("sysctl").arg("-n").arg("hw.model").output()
                    .context(DeviceName)?
                    .stdout
                    .iter()
                    .map(|b| model.push(*b as char))
                    .collect::<()>();
                self.model = model.trim().to_string();
            },
        }

        // trim junk
        self.model = self.model.clone()
            .replace("To", "")
            .replace("Not", "")
            .replace("Version", "")
            .replace("Applicable", "")
            .replace("Undefined", "")
            .replace("Specified", "")
            .replace("OEM", "")
            .replace("INVALID", "")
            .replace("Default", "")
            .replace("O.E.M", "")
            .replace("Product", "")
            .replace("Name", "")
            .replace("string", "")
            .replace("System", "")
            .trim().to_string();


        Ok(())
    }

    // format it
    pub fn format(&self) -> String { self.model.clone() }
}
