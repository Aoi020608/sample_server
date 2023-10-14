#![allow(dead_code, unused_variables)]

use std::{future::Future, io::BufRead, sync::{Arc, Mutex}};

mod tokio {
    pub async fn spawn(_: impl Future) {}
}

#[tokio::main]
async fn main() {
    let x = Arc::new(Mutex::new(0));
    let x1 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            *x1.lock() += 1;
        }
    });

    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            *x2.lock() -= 1;
        }
    });
}


