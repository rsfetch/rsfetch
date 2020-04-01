use crate::*;
use std::fs;
use std::process::Command;

pub struct KernelInfo {
    version: String,
}

impl KernelInfo {
    pub fn new() -> KernelInfo {
        KernelInfo {
            version: String::new(),
        }
    }

    pub fn get(&mut self) -> Result<()> {
        let path = "/proc/sys/kernel/osrelease";
        if fs::metadata(path).is_ok() {
            let f = fs::read_to_string(path).context(KernelVersion)?;
            self.version = f.trim().to_string();
        } else {
            let command = Command::new("uname")
                .arg("-r")
                .output()
                .context(KernelVersion)?;

            let output = std::str::from_utf8(&command.stdout)
                .unwrap();

            self.version = output.trim().into();
        }

        Ok(())
    }

    // format it
    pub fn format(&self) -> String {
        self.version.clone()
    }
}
