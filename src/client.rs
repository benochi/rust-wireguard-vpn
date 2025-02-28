use anyhow::{Result, Context};
use wgctrl_rs::{Handle, Key, DeviceConfigBuilder, PeerConfigBuilder, AllowedIp};
use crate::config::Config;

pub async fn run_client(iface_name: &str, config:Config) -> Result<()> {
    let handle = Handle::new()
        .map_err(|e| anyhow::anyhow!("Failed to create Wireguard handle: {}", e))?;
    
    let priv_key = Key::from_base64(&config.private_key)
        .map_err(|_| anyhow::anyhow!("Invalid client private key."))?;
    
    let server_pub = Key::from_base64(&config.public_key)
        .map_err(|_| anyhow::anyhow!("Invalid public key"))?;

    let peer_config = PeerConfigBuilder::new(&server_pub)
        .add_allowed_ip(AllowedIp::new("0.0.0.0/0".parse()
        .map_err(|e| anyhow::anyhow!("Invalid CIDR notation: {}", e))?)?)
        .set_endpoint(&config.endpoint)
        .build();
    
    let device_config = DeviceConfigBuilder::new()
        .set_private_key(&priv_key)
        .add_peer(peer_config)
        .build();
    
    handle.apply(iface_name, &device_config)
        .map_err(|e| anyhow::anyhow!("Failed to apply wireguard cleint config: {}", e))?;

    println!("Client successfully connected to {}", config.endpoint);
    tokio::signal::ctrl_c().await?;
    Ok(())
}