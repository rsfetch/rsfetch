use crate::*;
use std::process::Command;

pub struct MusicInfo {
    data: String,
}

impl MusicInfo {
    pub fn new() -> MusicInfo {
        MusicInfo {
            data: String::new(),
        }
    }

    pub fn get(&mut self) -> Result<()> {
        let data = Command::new("mpc")
            .arg("-f")
            .arg("%artist% - (%date%) %album% - %title%a")
            .output().context(Mpc)?;
        self.data = String::from_utf8_lossy(&data.stdout)
            .into_owned().split("\n").collect::<Vec<&str>>()[0].to_string();
        self.data.pop();

        Ok(())
    }

    // format it
    pub fn format(&self) -> String {
        self.data.clone()
    }
}
