#[allow(dead_code)]

use crate::*;
use std::vec::Vec;
use std::process::Command;

#[derive(Clone, Debug)]
pub enum PkgManager {
    Arch,
    Debian,
    Void,
    Fedora,
    BSD,
    Solus,
    Suse,
    Alpine,
    Gentoo,
    Pip,
    Cargo,
    Unknown,

    // TODO
    //CRUX,
    //KISS,
    //Slackware,
    //NixOS,
    //Bedrock,
}

pub struct PkgInfo {
    manager: Vec<PkgManager>,
    count:   usize,
}

impl PkgInfo {
    pub fn new() -> PkgInfo {
        PkgInfo {
            manager: Vec::new(),
            count:   0,
        }
    }

    pub fn get(&mut self) -> Result<()> {
        for manager in self.manager.clone() {
            let mut command = match manager {
                PkgManager::Arch    => Command::new("pacman -Qq"),
                PkgManager::Debian  => Command::new("apt list"),
                PkgManager::Void    => Command::new("xbps-query -l"),
                PkgManager::Fedora  => Command::new("dnf list --installed"),
                PkgManager::BSD     => Command::new("pkg info"),
                PkgManager::Suse    => Command::new("rpm -qa"),
                PkgManager::Solus   => Command::new("eopkg list-installed"),
                PkgManager::Alpine  => Command::new("apk info"),
                PkgManager::Gentoo  => Command::new("qlist -I"),
                PkgManager::Pip     => Command::new("pip list"),
                PkgManager::Cargo   => Command::new("cargo list"),
                PkgManager::Unknown => Command::new("echo -n ''"), // dummy
                //_                   => Command::new("echo -n ''"),
            };

            let stdout = command.output().context(Pkgcount)?.stdout;
            let mut count: usize = 0;
    
            let mut dest = "".to_owned();
            for byte in stdout {
                dest = format!("{}{}", dest, byte as char);
            }

            let _ = dest.split("\n").map(|_| count += 1).collect::<()>();
            self.count += count;
        }

        Ok(())
    }

    pub fn set_manager(&mut self, manager: &str) {
        let mngr: PkgManager = match manager {
            "pacman"     => PkgManager::Arch,
            "apt"        => PkgManager::Debian,
            "xbps"       => PkgManager::Void,
            "xbps-query" => PkgManager::Void,
            "dnf"        => PkgManager::Fedora,
            "pkg"        => PkgManager::BSD,
            "eopkg"      => PkgManager::Solus,
            "rpm"        => PkgManager::Suse,
            "apk"        => PkgManager::Alpine,
            "portage"    => PkgManager::Gentoo,
            "pip"        => PkgManager::Pip,
            "cargo"      => PkgManager::Cargo,
            _            => PkgManager::Unknown,
        };

        self.manager.push(mngr);
    }

    // format it
    pub fn format(&self) -> String {
        if self.count > 0 {
            return format!("{}", self.count);
        } else {
            return "nah!".to_owned();
        }
    }
}
