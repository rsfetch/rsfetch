use crate::*;
use std::fs;
use std::process::Command;

pub struct Hostname {
    name: String,
}

impl Hostname {
    pub fn new() -> Hostname {
        Hostname {
            name: String::new(),
        }
    }

    pub fn get(&mut self) -> Result<()> {
        if fs::metadata("/etc/hostname").is_ok() {
            let f = fs::read_to_string("/etc/hostname").context(ReadHostname)?;
            self.name = f.trim().to_string();
        } else {
            // fallback to `hostname` command
            let command = Command::new("hostname")
                .output()
                .context(ReadHostname)?;

            let hostname = String::from_utf8(command.stdout)
                .replace("\n", "")
                .unwrap();
                
            self.name = hostname;
        }

        Ok(())
    }

    // format it
    pub fn format(&self) -> String {
        self.name.clone()
    }
}
