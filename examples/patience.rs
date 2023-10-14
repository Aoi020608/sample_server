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
