#![allow(dead_code, unused_variables)]

use std::{future::Future, io::BufRead};

fn main() {}

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
        async { Response }
    }
}

