#![allow(dead_code, unused_variables)]

use std::{
    future::Future,
    io::{self, BufRead},
    pin::Pin,
    sync::{Arc, Mutex},
};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {

    let x = Arc::new(Mutex::new(0));
    let x1 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            let x = x1.lock();
            tokio::fs::read_to_string("file").await;

            *x1.lock().unwrap() += 1;
        }
    });

    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            let x = x2.lock();
            tokio::fs::read_to_string("file").await;

            *x2.lock().unwrap() += 1;
        }
    });
    

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

struct Request;
struct Response;

trait Service {
    type CallFuture: Future<Output = Response>;

    fn call(&mut self, _: Request) -> Self::CallFuture;
}

struct X;

impl Service for X {
    type CallFuture = Pin<Box<dyn Future<Output = Response>>>;
    fn call(&mut self, _: Request) -> Self::CallFuture {
        Box::pin(async move { Response })
    }
}
