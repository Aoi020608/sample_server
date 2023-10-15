#![allow(dead_code, unused_variables)]

use std::{
    future::Future,
    io::BufRead,
    sync::{Arc, Mutex},
};

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        println!("Hello, world!");

        let mut network = read_from_network();
        let mut termial = read_from_termianl();
        let mut foo = foo2();

        let mut f1 = tokio::fs::File::open("foo");
        let mut f2 = tokio::fs::File::open("bar");
        let copy = tokio::io::copy(&mut f1, &mut f2);

        select! {
            stream <- (&mut network).await => {
                // do something
            }
            line <- (&mut terminal).await => {
                // do something with line
                break;
            }
            foo <- (&mut foo).await => {}
            
        }

        // await means don't run the following lists of of instructions until foo1 actually resolved
        // into its output type
        let x = foo1().await;
        println!("foo2");
    });
}

async fn foo1() -> usize {
    println!("foo1");
    0
}

fn foo2() -> impl Future<Output = usize> {
    async { 0 }
}
