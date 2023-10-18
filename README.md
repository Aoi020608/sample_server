# sample-server

## Libraries

### futures
- general interface how to deal with asynchronous computation, interface for tasks, norifier, wake ups, executor.  
- it doesn't really define any implementations

### tokio
- executor crate

#### `tokio::spawn` 
- gives the future it is passed to the executor whenever it wishes concurrently with other futures.

#### `thread::spawn` 
- spawns a new operating system thread that will run in parallel with everything else in your program and is outside of the executor's control 
- does not take a future it takes a closure so 

## REFERENCES

examples/patience.rs
- [Crust of Rust: async/await](https://www.youtube.com/watch?v=ThjvMReOXYM)

examples/future.rs
- https://www.youtube.com/watch?v=9_3krAQtD2k

examples/mini_tokio.rs
- https://github.com/tokio-rs/website/blob/master/tutorial-code/mini-tokio/src/main.rs

blog
- https://without.boats/blog/why-async-rust/
