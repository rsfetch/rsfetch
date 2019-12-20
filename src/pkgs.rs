mod util;
use crate::util::*;
use crate::Error;
use std::vec::Vec;
use std::result::Result;

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

    //CRUX,
    //KISS,
    //Slackware,
    //NixOS,
    //Bedrock,

    Pip,
    Cargo,
    Unknown,
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

    pub fn get(&mut self) -> Result<(), Error> {
        for manager in self.manager {
            let command = match manager {
                Arch    => Command::new("pacman").arg("-Qq"),
                Debian  => Command::new("apt").arg("list"),
                Void    => Command::new("xbps-query").arg("-l"),
                Fedora  => Command::new("dnf").arg("list --installed"),
                BSD     => Command::new("pkg").arg("info"),
                Solus   => Command::new("eopkg").arg("list-installed"),
                Alpine  => Command::new("apk").arg("info"),
                Gentoo  => Command::new("qlist").arg("-I"),
                Pip     => Command::new("pip").arg("list"),
                Cargo   => Command::new("cargo").arg("list"),
                Unknown => Command::new("echo").arg("-n"), // dummy
            };

            let stdout = command.output().context(Pkgcount)?;
            self.count += count_lines(stdout);
        }

        Ok(())
    }

    pub fn set_manager(&mut self, manager: &str) {
        let mngr: PkgManager = match manager {
            "pacman"  => Arch,
            "apt"     => Debian,
            "xbps"    => Void,
            "dnf"     => Fedora,
            "pkg"     => BSD,
            "eopkg"   => Solus,
            "rpm"     => Suse,
            "apk"     => Alpine,
            "portage" => Gentoo,
            "pip"     => Pip,
            "cargo"   => Cargo,
            _ => Unknown,
        }

        self.manager.push(mngr);
    }

    // format it
    pub fn format(&self) -> String {
        if self.count > 0 {
            format!("{}", self.count);
        } else {
            "nah!".to_owned()
        }
    }
}
