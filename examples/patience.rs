#![allow(dead_code, unused_variables)]

use std::{
    future::Future,
    io::BufRead,
    sync::{Arc, Mutex},
};

use tokio::net::TcpListener;

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        println!("Hello, world");

        let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080");
        let mut connections = futures::future::FuturesUnordered::new();
        loop {
            tokio::select! {
                stream = accept => {
                    if let Ok(st) = stream {
                        connections.push(handle_connection(st));
                    }
                }
                _ = (&mut connections).await => {}
            }
        }
    });
}

async fn handle_connection(_: TcpListener) {
    todo!()
}
