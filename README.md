# sample-server

## Libraries

### Tokio
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

blog
- https://without.boats/blog/why-async-rust/
