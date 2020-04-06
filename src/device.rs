use crate::*;
use std::fs;
use std::process::Command;

pub struct DeviceInfo {
    model: String,
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        DeviceInfo {
            model: String::new(),
        }
    }

    pub fn get(&mut self) -> Result<()> {
        let mut path = "/sys/devices/virtual/dmi/id/product_name";
        if fs::metadata(path).is_err() {
            path = "/sys/firmware/devicetree/base/model";
        }

        let f = fs::read_to_string(path);
        match f {
            Ok(c) => self.model = c.trim().trim_matches(char::from(0)).to_string(),
            Err(_) => {
                // fallback to sysctl...
                let command = Command::new("sysctl")
                    .arg("-n")
                    .arg("hw.model")
                    .output()
                    .context(DeviceName)?;

                let model = std::str::from_utf8(&command.stdout)
                    .unwrap()
                    .replace("\n", "");

                self.model = model.trim().into();
            }
        }

        // trim junk
        self.model = self
            .model
            .clone()
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
            .trim()
            .to_string();

        Ok(())
    }

    // format it
    pub fn format(&self) -> String {
        self.model.clone()
    }
}
