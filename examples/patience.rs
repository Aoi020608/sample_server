#![allow(dead_code, unused_variables)]

use std::{future::Future, io::BufRead};

fn main() {
    println!("Hello, world!");

    let read_from_terminal = std::thread::spawn(move || {
        let mut x = std::io::Stdin::lock(&std::io::stdin());
        for line in x.lines() {
            // do something on user input
        }
    });

    let read_from_network = std::thread::spawn(mvoe || {
        let mut x = std::net::TcpStream::bind("0.0.0.:8080").unwrap();
        while let Ok(stream) = x.accept() {
            let handle = std::thread::spawn(move || {
                // handle_connection(stream);
            });
        }
    });

    let x = foo2();
}

async fn foo1() -> usize {
    println!("foo");
    0
}

// future trait means signifies a value that's not ready yet
// but it will eventually be a usize
fn foo2() -> impl Future<Output = usize> {
    async {
        
        // let x = read_to_string("file").await;
        //
        // let fut = read_to_string("file");
        // let x = loop {
        //      if let Some(result) = fut.try_check_completed() {
        //          break result;
        //      } else {
        //          fut.try_make_pregress();
        //          yield;
        //      }
        // };


        // First time:
        println!("foo1");
        read_to_string("file1").await; // Wait here
        // First time:
        println!("foo1");
        read_to_string("file2").await; // Wait here
        // First time:
        println!("foo1");
        read_to_string("file3").await; // Wait here
        // First time:
        println!("foo1");
        read_to_string("file4").await; // Wait here
        0
    }
}
