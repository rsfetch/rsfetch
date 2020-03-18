// various utility functions
use crate::*;
use std::process::Command;

#[derive(Clone, PartialEq)]
pub enum OS {
    Linux,
    FreeBSD,
    OpenBSD,
    NetBSD,
    DragonflyBSD,
    Other,
}

pub struct OSInfo {
    #[allow(dead_code)]
    os: OS,
}

impl OSInfo {
    pub fn get_os() -> Result<OS, std::io::Error> {
        let mut uname = String::new();
        Command::new("uname")
            .arg("-s")
            .output()?
            .stdout
            .iter()
            .for_each(|b| uname.push(*b as char));
        let os = match uname.replace("\n", "").trim().as_ref() {
            "Linux" => OS::Linux,
            "FreeBSD" => OS::FreeBSD,
            "NetBSD" => OS::NetBSD,
            "OpenBSD" => OS::OpenBSD,
            "DragonFly" => OS::DragonflyBSD,
            &_ => OS::Other,
        };

        Ok(os)
    }
}
