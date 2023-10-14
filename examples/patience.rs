#![allow(dead_code, unused_variables)]

use std::{future::Future, io::BufRead};

fn main() {
    let runtime = tokio::runtime::Runtime::new();
    runtime.block_on(async {
        println!("Hello world!");

        let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080");
        while let Ok(stream) = accept.await {
            tokio::spawn(|| handle_connection(stream));
        }
    });
}

async fn handle_connection(_: TcpStream) {
    let x = Arc::new(Mutex::new(vec![]));
    let x1 = Arc::clone(&x);
    let join_handle = tokio::spawn(async move {
        x1.lock();
    });

    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        x2.lock();
    });
}

async fn foo1() -> usize {
    println!("foo");
    0
}

async fn read_to_string(s: &str) -> &str {
    ""
}

fn expensive_function(x: &str) {}

// future trait means signifies a value that's not ready yet
// but it will eventually be a usize
fn foo2(cancel: tokio::sync::mpsc::Receiver<()>) -> impl Future<Output = usize> {
    async {
        println!("foo1");
        read_to_string("file1").await;
        println!("foo1");
        /*
        race! {
            done <- read_to_string("file2").await => {
                // continue; fall-through to println below
            }
            cancel <- cancel.await => {
                return 0;
            }
        };
        */
        println!("foo3");

        let x = read_to_string("file3").await;
        println!("file1");
        expensive_function(x);
        println!("file1");
        println!("file1");
        0
    }
}
