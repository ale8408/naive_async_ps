use std::net::Ipv4Addr;
use futures::{executor, stream::FuturesUnordered, StreamExt};
// asynchronous connections
use async_std::net::TcpStream;
// naive benchmarking
use std::time::Instant;

mod arguments_manager;

mod types_manager;
use types_manager::*;

fn main() {
    let start = Instant::now();

    let (target_ip, target_range) = arguments_manager::args_to_vars();
    
    let open_ports = executor::block_on(try_connect_range(target_ip, target_range));
    println!("Open ports: {open_ports:#?}");

    let elapsed = start.elapsed();
    println!("It took: {elapsed:?}");
}

async fn try_connect_range(
    target_ip: Ipv4Addr,
    target_range: Vec<PortRange>
) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();
    let mut futures = FuturesUnordered::new();

    for range in target_range {
        for target_port in range.start..=range.end {
            futures.push(try_connection(target_ip, target_port));
        }
    }

    while let Some(result) = futures.next().await {
        match result {
            Some(port) => open_ports.push(port),
            None => {},
        }
    }

    open_ports
}

async fn try_connection(
    target_ip: Ipv4Addr,
    target_port: u16,
) -> Option<u16> {
    return match TcpStream::connect(format!("{target_ip}:{target_port}")).await {
        Ok(_) => Some(target_port),
        Err(_) => None,
    }
}