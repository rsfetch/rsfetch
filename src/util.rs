// various utility functions
use crate::*;
use std::vec::Vec;
use std::process::Command;

#[derive(Clone, PartialEq)]
enum OS {
    Linux,
    FreeBSD,
    OpenBSD,
    NetBSD,
    Other
}

struct OSInfo {
    os: OS,
}

impl OSInfo { 
    fn get_os() -> Result<OS, std::io::Error> {
        let mut uname = String::new();
        Command::new("uname").arg("-s")
            .output()?.stdout.iter()
            .for_each(|b| uname.push(*b as char));
        let os = match uname.as_ref() {
            "Linux"   => OS::Linux,
            "FreeBSD" => OS::FreeBSD,
            "NetBSD"  => OS::NetBSD,
            "OpenBSD" => OS::OpenBSD,
            &_        => OS::Other,
        }
    }
}
