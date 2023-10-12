#![allow(dead_code, unused_variables)]

use std::future::Future;

fn main() {
    println!("Hello, world!");

    let x = foo1();
}

async fn foo1() -> usize {
    println!("foo");
    0
}

// future trait means signifies a value that's not ready yet
// but it will eventually be a usize
fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo");
        0
    }
}
