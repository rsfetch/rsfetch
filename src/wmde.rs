use std::env;
use crate::*;

pub struct WMDEInfo {
    de: String,
    wm: String,
}

impl WMDEInfo {
    pub fn new() -> WMDEInfo {
        DeviceInfo {
            de: String::new(),
            wm: String::new(),
        }
    }

    pub fn get(&mut self) -> Result<(), Error> {
        if let Some(de) = env::var("XDG_DESKTOP_SESSION")
            .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
            .or_else(|_| env::var("DESKTOP_SESSION"))
        {
            self.de = de;
        }

        let path = format!("{}.xinitrc", dirs::home_dir().context(HomeDir)?);
        let file = File::open(path).context(OpenXInitRc)?;
        let reader = BufReader::new(file);
        let last_line = reader.lines().last()
            .context(EmptyXInitRc)?
            .context(ReadXInitRc)?;
        let space = last_line.fine(' ').context(GuessWm)?;
        self.wm = last_line[space + 1..].to_string();
        
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
