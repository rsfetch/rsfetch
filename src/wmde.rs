use std::env;
use crate::*;

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

    pub fn get(&mut self) -> Result<(), Error> {
        let de = env::var("XDG_DESKTOP_SESSION")
            .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
            .or_else(|_| env::var("DESKTOP_SESSION"));
        self.de = de.unwrap();

        let path = format!("{}.xinitrc", dirs::home_dir()
                           .context(HomeDir)?
                           .to_str()
                           .unwrap());
        let file = File::open(path).context(OpenXInitRc)?;
        let reader = BufReader::new(file);
        let last_line = reader.lines().last()
            .context(EmptyXInitRc)?
            .context(ReadXInitRc)?;
        let space = last_line.find(' ').context(GuessWm)?;
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
