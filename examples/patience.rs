#![allow(dead_code, unused_variables)]

use std::{future::Future, io::BufRead};

fn main() {
    println!("Hello, world!");

    // one future
    let read_from_terminal = std::thread::spawn(move || {
        let mut x = std::io::Stdin::lock(&std::io::stdin());
        for line in x.lines() {
            // do something on user input
        }
    });

    // two future
    let read_from_network = std::thread::spawn(move || {
        let mut x = std::net::TcpListener::bind("0.0.0.0:8080").unwrap();
        while let Ok(stream) = x.accept() {
            let handle = std::thread::spawn(move || {
                // handle_connection(stream);
            });
        }
    });

    let network = read_from_network();
    let termianl = read_from_terminal();

    let x = foo2();
}

async fn foo1() -> usize {
    println!("foo");
    0
}

async fn read_to_string(s: &str) -> &str {""}

fn expensive_function(x: &str) {}

// future trait means signifies a value that's not ready yet
// but it will eventually be a usize
fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo1");
        read_to_string("file1").await;
        println!("foo2");
        read_to_string("file2").await;
        println!("foo3");
        let x = read_to_string("file3").await;
        println!("file1");
        expensive_function(x);
        println!("file1");
        println!("file1");
        0
    }
}
