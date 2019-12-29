use std::fs;
use crate::*;
use std::process::Command;

pub struct KernelInfo {
    version: String,
}

impl KernelInfo {
    pub fn new() -> KernelInfo {
        KernelInfo { version: String::new(), }
    }

    pub fn get(&mut self) -> Result<()> {
        let path = "/proc/sys/kernel/osrelease";
        if fs::metadata(path).is_ok() {
            let f = fs::read_to_string(path)
                .context(KernelVersion)?;
            self.version = f.trim().to_string();
        } else {
            let mut output: String = String::new();
            let _ = Command::new("uname").arg("-r")
                .output().context(KernelVersion)?
                .stdout.iter().map(|b| output.push(*b as char))
                .collect::<()>();
            self.version = output.trim().to_string();
        }

        Ok(())
    }

    // format it
    pub fn format(&self) -> String { self.version.clone() }
}
