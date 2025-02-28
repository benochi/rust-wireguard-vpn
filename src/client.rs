use anyhow::{Result, Context};
use wgctrl_rs::{Interface, Key};
use crate::config::Config;

pub async fn run_client(iface_name: &str, config:Config) -> Result<()> {
    let mut wg = Interface::new(iface_name)
        .context("Failed to create client Wireguard interface")?;
    let priv_key = Key::from_base64(&config.private_key)
        .context("Invalid private key")?;
    wg.set_secret_key(&priv_key)?;
    let server_pub = Key::from_base64(&config.public_key)
        .context("Invalid server public key")?;
    wg.add_peer(&server_pub, Some(&config.endpoint), &config.allowed_ips)?;
    wg.apply().context("Failed to apply Config.")?;
    println!("Client successfully connected to {}", config.endpoint);
    tokio::signal::ctrl_c().await?;
    Ok(())
}