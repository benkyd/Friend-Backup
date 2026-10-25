// peers.rs
// Here we manage connections (incoming and outbound)

use crate::configuration;
use crate::friends;

use std::error::Error;
use tokio::net::TcpListener;

pub async fn listen(settings: configuration::Settings) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:9090").await?;
    println!("Listening for incoming connections");

    match listener.accept().await {
        Ok((_socket, addr)) => println!("new client: {:?}", addr),
        Err(e) => println!("couldn't get client: {:?}", e),
    }

    Ok(())
}
