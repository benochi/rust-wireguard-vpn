use anyhow::{Result, Context};
use wgctrl_rs::{Handle, Key, DeviceConfigBuilder};
use crate::config::Config;

pub async fn run_server(iface_name: &str, config: Config) -> Result<()> {
    let handle = Handle::new()
        .map_err(|e| anyhow::anyhow!("Failed to create WireGuard handle: {}", e))?;

    let priv_key = Key::from_base64(&config.private_key)
        .map_err(|_| anyhow::anyhow!("Invalid Private Key"))?;
    
    let device_config = DeviceConfigBuilder::new
        .set_private_key(&priv_key)
        .set_listen_port(config.port)
        .build();

    handle.apply(iface_name, &device_config)
        .map_err(|e| anyhow::anyhow!("Failed to apply Wireguard server config: {}", e))?;

    println!("Server running on {}", config.endpoint);
    tokio::signal::ctrl_c().await?;
    Ok(())
}