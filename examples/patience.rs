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

    let mut network = read_from_network();
    let mut termial = read_from_network();
    let mut foo = foo2();

    let mut f1 = tokio::fs::File::open("foo");
    let mut f2 = tokio::fs::File::create("bar");
    let copy = tokio::io::copy(&mut f1, &mut f2);

    /*
    select! {
        stream <- network.await => {
            // do something on stream
        }
        line <- terminal.await => {
            // do something with line
        }
        foo <- foo2().await => {
        }
        _ <- copy.await => {
        }
    };
    */

    // let x = foo2();

    let files: Vec<_> = (0..3).map(|i| tokio::fs::read_to_string(format!("file{}", 1))).collect();

    // compare
    let file1 = files[0].await;
    let file2 = files[1].await;
    let file3 = files[2].await;

    // to this
    // join macro is comvineent when deal with a few things
    let (file1, file2, file3) = join!(files[0], files[1], files[2]);

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
