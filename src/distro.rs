use crate::*;
use std::fs;
use std::process::Command;

pub struct DistroInfo {
    name: String,
    pretty_name: String,
    id: String,
    distrib_id: String,
}

impl DistroInfo {
    pub fn new() -> DistroInfo {
        DistroInfo {
            name: String::new(),
            pretty_name: String::new(),
            id: String::new(),
            distrib_id: String::new(),
        }
    }

    pub fn get(&mut self) -> Result<()> {
        // check for Bedrock
        if fs::metadata("/bedrock/etc/os-release").is_ok() {
            self.name = "bedrock".to_string();
            self.pretty_name = "Bedrock Linux".to_string();

            return Ok(());
        }

        // check for CRUX
        let isthiscrux = Command::new("crux").output();
        match isthiscrux {
            Ok(__) => {
                // TODO: parse output of `crux` command
                // into self.name and self.pretty_name
                self.name = "crux".to_string();
                self.pretty_name = "CRUX Linux".to_string();

                return Ok(());
            }
            Err(_) => (),
        }

        // check for GNU Guix
        let isthisguix = Command::new("guix").output();
        match isthisguix {
            Ok(_) => {
                self.name = "guix".to_string();
                self.pretty_name = "Guix System".to_string();

                return Ok(());
            }
            Err(_) => (),
        }

        // check for /etc/os-release file
        if fs::metadata("/etc/os-release").is_ok() {
            let file = fs::read_to_string("/etc/os-release").context(OsRelease)?;

            for value in file.split('\n') {
                let keyval = value.split('=').collect::<Vec<&str>>();
                if keyval.len() < 2 {
                    continue;
                }

                let key = keyval[0].trim();
                let val = keyval[1].trim().trim_matches('"');

                match key {
                    "NAME" => self.name = val.to_string(),
                    "ID" => self.id = val.to_string(),
                    "DISTRIB_ID" => self.distrib_id = val.to_string(),
                    "PRETTY_NAME" => self.pretty_name = val.to_string(),
                    &_ => (),
                }
            }

            return Ok(());
        }

        // check for /usr/lib/os-release file
        if fs::metadata("/usr/lib/os-release").is_ok() {
            let file = fs::read_to_string("/usr/lib/os-release").context(OsRelease)?;

            for value in file.split('\n') {
                let keyval = value.split('=').collect::<Vec<&str>>();
                if keyval.len() < 2 {
                    continue;
                }

                let key = keyval[0].trim();
                let val = keyval[1].trim().trim_matches('"');

                match key {
                    "NAME" => self.name = val.to_string(),
                    "ID" => self.id = val.to_string(),
                    "DISTRIB_ID" => self.distrib_id = val.to_string(),
                    "PRETTY_NAME" => self.pretty_name = val.to_string(),
                    &_ => (),
                }
            }

            Ok(())
        } else {
            // just return the output of uname -sr ;P
            // also handles the BSD's
            let uname = Command::new("uname").arg("-s").output();
            match uname {
                Ok(out) => {
                    let output = String::from_utf8(out.stdout)
                        .unwrap()
                        .replace("\n", "");

                    self.name = output;
                    Ok(())
                }

                Err(_) => {
                    self.name = "?".to_string();
                    Ok(())
                }
            }
        }
    }

    pub fn format(&self) -> String {
        if self.pretty_name != "" {
            self.pretty_name.clone()
        } else {
            self.name.clone()
        }
    }
}
