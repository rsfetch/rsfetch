use std::result::Result;

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

    pub fn get(&mut self) -> Result<(), reqwest::Error> {
        self.ip_address = request::get("https://ipecho.net/plain")?.text()?;
        Ok(())
    }

    // format it
    pub fn format(&self) -> String { self.ip_address.clone() }
}
