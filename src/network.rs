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
        NetworkInfo {
            ip_address: String::new(),
        }
    }

    pub async fn get(&mut self) -> Result<()> {
        let client = hyper::Client::new();
        let resp = client.get(hyper::Uri::from_static("http://ipecho.net/plain"))
                    .await
                    .context(Hyper)?;
        let buf = hyper::body::to_bytes(resp)
                    .await
                    .context(Hyper)?;
        self.ip_address = std::str::from_utf8(&buf).unwrap().to_string();
        Ok(())
    }

    // format it
    pub fn format(&self) -> String {
        self.ip_address.clone()
    }
}
