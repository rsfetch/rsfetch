use std::fs;
use std::result::Result;

pub struct DistroInfo {
    name:        String,
    pretty_name: String,
    id:          String,
    distrib_id:  String,
}

impl DistroInfo {
    pub fn new() -> DistroInfo {
        DistroInfo {
            name:        String::new(),
            pretty_name: String::new(),
            id:          String::new(),
            distrib_id:  String::new(),
        }
    }

    // TODO: support for distros like CRUX, which
    // typically don't have an /etc/os-release file.
    pub fn get(&mut self) -> Result<(), std::io::Error> {
        let values = fs::read_to_string("/etc/os-release")?
            .split("\n").collect::<Vec<&str>>();

        for value in values {
            let key = value.split("=")[0].trim();
            let val = value.split("=")[1].trim()
                .trim_matches("\"");

            match key {
                "NAME"        => self.name = val.to_string(),
                "ID"          => self.id   = val.to_string(),
                "DISTRIB_ID"  => self.distrib_id = val.to_string(),
                "PRETTY_NAME" => self.pretty_name = val.to_string(),
            }
        }

        Ok(())
    }

    pub fn format(&self) -> String {
        self.pretty_name.or(self.name)
    }
}
