use std::env;
use crate::*;
use std::io::{ BufReader, BufRead };

pub struct WMDEInfo {
    de: String,
    wm: String,
}

impl WMDEInfo {
    pub fn new() -> WMDEInfo {
        WMDEInfo {
            de: String::new(),
            wm: String::new(),
        }
    }

    pub fn get(&mut self) -> Result<()> {
        let de = env::var("XDG_DESKTOP_SESSION")
            .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
            .or_else(|_| env::var("DESKTOP_SESSION"));

        match de {
            Ok(d) => self.de = d,
            Err(_) => (),
        }

        // if Err() is returned anywhere, it will be returned right
        // here.
        let path = format!("{}/.xinitrc", dirs::home_dir()
                           .context(HomeDir)?
                           .to_str()
                           .unwrap());
        let file = File::open(path).context(OpenXInitRc)?;
        let reader = BufReader::new(file);
        let last_line = reader.lines().last()
            .context(EmptyXInitRc)?
            .context(ReadXInitRc)?;

        if let Some(wm) = last_line.split(" ").last() {
            self.wm = wm.trim().to_string();
        } else {
            self.wm = "?".to_string();
        }
        
        Ok(())
    }

    // format it
    pub fn format(&self) -> String {
        if self.de != "" {
            return self.de.clone()
        } else {
            return self.wm.clone()
        }
    }
}
