use anyhow::{Result, Context};
use wgctrl_rs::{Interface, Key};
use crate::config::Config;

pub async fn run_server(iface_name: &str, config: Config) -> Result<()> {
    let mut wg = Interface::new(iface_name)
        .context("Failed to create a Wireguard interface.")?;
    let priv_key = Key::from_base64(&config.private_key)
        .context("Invalid Private key.")?;
    wg.set_secret_key(&priv_key)?;
    wg.set_listen_port(51820)?;
    println!("Server running on {}", config.endpoint);
    tokio::signal::ctrl_c().await?;
    Ok(())
}