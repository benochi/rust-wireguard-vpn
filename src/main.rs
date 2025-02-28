use anyhow::Result;
mod config;
mod server;
mod client;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len()< 3 {
        println!("Usage: {} [server|client] <host_server_ip> <port>", args[0]);
        return Ok(());
    }

    let host_server_ip = &args[2];
    let port: u16 = args[3].parse().map_err(|_| anyhow::anyhow!("Invalid port number!"))?;

    match args[1].as_str() {
        "server" => {
            let config = config::Config::new_server(host_server_ip, port);
            server::run_server("wg0", config).await?;
        }
        "client" =>{
            let config = config::Config::new_client(
                "jK9L1mN3oP5qR7sT9uV2wX4yZ6aB8cD0eF2gH4iJ6=", // Server pubkey
                host_server_ip,
                port
            );
            client::run_client("wg1", config).await?;
        }
        _ => println!("Use 'server' or 'client'."),
    }
    Ok(())
}