pub struct ClientConfig {
    pub server_addr: String,
}

impl ClientConfig {
    pub fn default() -> Self {
        ClientConfig {
            server_addr: "127.0.0.1:8080".to_string(),
        }
    }
}
