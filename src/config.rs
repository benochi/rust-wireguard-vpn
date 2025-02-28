pub struct Config {
    pub private_key: String,
    pub public_key:String,
    pub endpoint: String,
    pub allowed_ips: String,
    pub port: u16,
}

impl Config {
    pub fn new_server(host_server_ip: &str, port: u16) -> Self {
        Config {
            private_key: "eF8P+5gY8Q5zKx9v7mJ2nL3kR4tS6uW8yX0Z1aB9cD=".to_string(),
            public_key: "example_public_string".to_string(),
            endpoint: format!("{}:{}", host_server_ip, port),
            allowed_ips: "10.0.0.0/24".to_string(),
            port,
        }
    }

    pub fn new_client(_server_pub: &str, server_ip: &str, port: u16) -> Self {
        Config {
            private_key: "Client_private_key".to_string(),
            public_key: "Client_public_key".to_string(),
            endpoint: format!("{}:{}", server_ip, port),
            allowed_ips: "0.0.0.0/24".to_string(),
            port
        }
    }

}