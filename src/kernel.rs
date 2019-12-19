use std::fs;
use std::result::Result;

pub struct KernelInfo {
    version: String,
}

impl KernelInfo {
    pub fn new() -> KernelInfo {
        KernelInfo { version: String::new(), }
    }

    pub fn get(&mut self) -> Result<(), std::io::Error> {
        let f = fs::read_to_string("/proc/sys/kernel/osrelease")?;
        self.version = f.trim().to_string();

        Ok(())
    }

    // format it
    pub fn format(&self) -> String { self.version.clone() }
}
