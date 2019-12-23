use std::fs;
use crate::*;

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
        let f = fs::read_to_string("/etc/hostname")
            .context(Hostname)?;
        self.model = f.trim().to_string();

        Ok(())
    }

    // format it
    pub fn format(&self) -> String { self.name.clone() }
}
