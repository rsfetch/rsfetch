#[allow(dead_code)]
use crate::*;
use std::process::Command;
use std::vec::Vec;

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
    count: usize,
}

impl PkgInfo {
    pub fn new() -> PkgInfo {
        PkgInfo {
            manager: Vec::new(),
            count: 0,
        }
    }

    pub fn get(&mut self) -> Result<()> {
        for manager in self.manager.clone() {
            let output = match manager {
                PkgManager::Arch => Command::new("pacman")
                    .arg("-Q")
					.arg("-q")
                    .output()
                    .context(Pkgcount)?,
                PkgManager::Debian => Command::new("apt")
                    .arg("list")
					.arg("--installed")
                    .output()
                    .context(Pkgcount)?,
                PkgManager::Void => Command::new("xbps-query")
                    .arg("-l")
                    .output()
                    .context(Pkgcount)?,
                PkgManager::Fedora => Command::new("dnf")
                    .arg("list")
					.arg("installed")
                    .output()
                    .context(Pkgcount)?,
                PkgManager::BSD => Command::new("pkg")
					.arg("info")
					.output()
					.context(Pkgcount)?,
                PkgManager::Suse => Command::new("rpm")
					.arg("-q")
					.arg("-a")
					.output()
					.context(Pkgcount)?,
                PkgManager::Solus => Command::new("eopkg")
                    .arg("list-installed")
                    .output()
                    .context(Pkgcount)?,
                PkgManager::Alpine => Command::new("apk")
					.arg("info")
					.output()
					.context(Pkgcount)?,
                PkgManager::Gentoo => Command::new("qlist")
					.arg("-I")
					.output()
					.context(Pkgcount)?,
                PkgManager::Pip => Command::new("pip")
					.arg("list")
					.output()
					.context(Pkgcount)?,
                PkgManager::Cargo => Command::new("cargo")
                    .arg("list")
                    .output()
                    .context(Pkgcount)?,
                PkgManager::Unknown => Command::new("echo")
                    .output()
                    .context(Pkgcount)?,
                //_                 => Command::new("echo -n ''"),
            };

            // count lines in stdout
            let mut count: usize = 0;
            output
                .stdout
                .iter()
                .map(|b| {
                    if (*b as usize) == 10 {
                        count += 1;
                    }
                })
                .collect::<()>();

            self.count += count;
        }

        Ok(())
    }

    pub fn set_manager(&mut self, manager: &str) {
        let mngr: PkgManager = match manager {
            "pacman" => PkgManager::Arch,
            "apt" => PkgManager::Debian,
            "xbps" => PkgManager::Void,
            "xbps-query" => PkgManager::Void,
            "dnf" => PkgManager::Fedora,
            "pkg" => PkgManager::BSD,
            "eopkg" => PkgManager::Solus,
            "rpm" => PkgManager::Suse,
            "apk" => PkgManager::Alpine,
            "portage" => PkgManager::Gentoo,
            "pip" => PkgManager::Pip,
            "cargo" => PkgManager::Cargo,
            _ => PkgManager::Unknown,
        };

        self.manager.push(mngr);
    }

    // format it
    pub fn format(&self) -> String {
        if self.count > 0 {
            return format!("{}", self.count);
        } else {
            "nah!".to_owned()
        }
    }
}
