pub mod log;
use std::net::TcpListener;

use crate::log;

pub fn run(input_port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", input_port))?;
    for stream in listener.incoming() {
        match stream {
            Ok(v) => {
                log!("New Connection: {}", v.peer_addr().unwrap_or(std::net::SocketAddr::new(std::net::IpAddr::V6(std::net::Ipv6Addr::new(63, 63, 63, 63, 63, 63, 63, 63)), 0)))
            },
            Err(e) => {
                log!("Connection error: {}", e)
            }
        }
    }
    Ok(())
}