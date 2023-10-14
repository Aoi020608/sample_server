#![allow(dead_code, unused_variables)]

use std::{future::Future, io::BufRead};

fn main() {}

enum StateMachine {
    Chunk1 {
        x: [u8; 1024],
        fut: tokio::fs::ReadIntoFuture<'x>,
    },
    Chunk2 {},
}

async fn foo() {
    // chunk1
    {
        let mut x = [0; 1024];
        let fut = tokio::fs::read_into("file.dat", &mut x[..]);
    }

    // fut.await
    yield; // really return;

    {
        let n = fut.output();
        println!("{:?}", x[..n]);
    }
}
