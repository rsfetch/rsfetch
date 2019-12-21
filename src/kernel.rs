use std::fs;
use crate::*;

pub struct KernelInfo {
    version: String,
}

impl KernelInfo {
    pub fn new() -> KernelInfo {
        KernelInfo { version: String::new(), }
    }

    pub fn get(&mut self) -> Result<()> {
        let f = fs::read_to_string("/proc/sys/kernel/osrelease")
            .context(KernelVersion)?;
        self.version = f.trim().to_string();

        Ok(())
    }

    // format it
    pub fn format(&self) -> String { self.version.clone() }
}
