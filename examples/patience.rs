#![allow(dead_code, unused_variables)]

use std::{
    future::Future,
    io::{self, BufRead},
    sync::{Arc, Mutex},
};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection<T>(socket: T) {
    let x: Vec<u8> = vec![1, 2];
    tokio::spawn(async move {
        //
        let y: &Vec<_> = &x;
        todo!()
    });

    // illegal
    // println!("{:?}", x);
}
