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
            let mut hostname = String::new();
            Command::new("hostname")
                .output()
                .context(ReadHostname)?
                .stdout
                .iter()
                .map(|b| hostname.push(*b as char))
                .collect::<()>();
            self.name = hostname.trim().to_string();
        }

        Ok(())
    }

    // format it
    pub fn format(&self) -> String {
        self.name.clone()
    }
}
