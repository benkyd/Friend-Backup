// peers.rs
// Here we manage connections (incoming and outbound)

use crate::configuration;
use crate::friends;

use std::{collections::HashSet, error::Error, io, net::SocketAddr, sync::{Arc, Mutex}, time::Duration};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpSocket, TcpStream}, time::sleep};

type Peers = Arc<Mutex<HashSet<SocketAddr>>>;

fn sock(settings: &configuration::Settings) -> io::Result<TcpSocket> {
    let s = TcpSocket::new_v4()?;
    s.set_reuseaddr(true)?;
    s.set_reuseport(true)?;
    s.bind(SocketAddr::from(([0,0,0,0], settings.listen)))?;
    Ok(s)
}

pub async fn listen(settings: &configuration::Settings) -> Result<(), Box<dyn Error>> {
    let peers: Peers = Default::default();
    let listener = sock(settings)?.listen(1024)?;

    for (peername, peer) in &settings.friends {
        println!("Dialing {}", peername);
        let settingsclone = settings.clone();
        tokio::spawn(dial(settingsclone, peers.clone(), SocketAddr::new(peer.ip, 9090)));
    }

    loop {
        let (s, addr) = listener.accept().await?;
        tokio::spawn(handle(peers.clone(), s, addr));
    }
}

async fn dial(settings: &configuration::Settings, peers: Peers, addr: SocketAddr) {
    loop {
        if !peers.lock().unwrap().contains(&addr) {
            if let Ok(s) = async { sock(settings)?.connect(addr).await }.await {
                handle(peers.clone(), s, addr).await;
            }
        }
        sleep(Duration::from_secs(5)).await;
    }
}

const PING: u8 = 0;
const BLOB: u8 = 1;

async fn handle(peers: Peers, mut s: TcpStream, addr: SocketAddr) {
    if s.local_addr().ok() == Some(addr) {
        return; // dialed ourselves
    }

    println!("up {addr}");

    if let Err(e) = session(&mut s).await {
        println!("{addr}: {e}");
    }

    peers.lock().unwrap().remove(&addr);
    println!("down {addr}");
}

async fn session(s: &mut TcpStream) -> io::Result<()> {
    write_frame(s, PING, &[]).await?;
    loop {
        let (op, body) = read_frame(s).await?;
        match op {
            PING => println!("ping!"),
            BLOB => { /* body */ }
            _ => return Err(io::ErrorKind::InvalidData.into()),
        }
    }
}

async fn read_frame(s: &mut TcpStream) -> io::Result<(u8, Vec<u8>)> {
    let op = s.read_u8().await?;
    let len = s.read_u32().await? as usize;
    if len > 16 << 20 {
        return Err(io::ErrorKind::InvalidData.into());
    }
    let mut body = vec![0; len];
    s.read_exact(&mut body).await?;
    Ok((op, body))
}

async fn write_frame(s: &mut TcpStream, op: u8, body: &[u8]) -> io::Result<()> {
    s.write_u8(op).await?;
    s.write_u32(body.len() as u32).await?;
    s.write_all(body).await
}
