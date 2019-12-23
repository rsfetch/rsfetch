use crate::*;

pub struct NetworkInfo {
    ip_address: String,

    // TODO: implement
    // interface: String,
    // is_connected: bool,
    // network_name: String,
    // upload_speed: usize,
    // download_speed: usize,
}

impl NetworkInfo {
    pub fn new() -> NetworkInfo {
        NetworkInfo { ip_address: String::new(), }
    }

    pub fn get(&mut self) -> Result<()> {
        self.ip_address = reqwest::get("https://ipecho.net/plain")
            .context(Reqwest)?
            .text()
            .context(Reqwest)?;
        Ok(())
    }

    // format it
    pub fn format(&self) -> String { self.ip_address.clone() }
}
