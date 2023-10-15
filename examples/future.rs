use std::net::TcpStream;

fn main() {
    /*
    let x = TcpStream::connect("127.0.0.1").unwrap();
    let y = TcpStream::connect("127.0.0.1").unwrap();

    x.write("foobar");
    y.write("foobar");

    assert_eq!(x.read(), "barfoo");
    assert_eq!(y.read(), "barfoo");
    */

    /*
    future
    let fut_x = TcpStream::connect("127.0.0.1")
        .and_then(|c| c.write("foobar"))
        .and_then(|c| c.read())
        .and_then(|(c, b)| b == "barfoo");
    println!("{:?}", fut);

    let fut_y = TcpStream::connect("127.0.0.1")
        .and_then(|c| c.write("foobar"))
        .and_then(|c| c.read())
        .and_then(|(c, b)| b == "barfoo");
    println!("{:?}", fut);

    */

    /*
    let a: Executor;
    let x = a.run(fut_x);
    let y = a.run(fut_y);


    let xy = a.run_all(vec![fut_x, fut_y]);
    */
}

/*
struct Executor;

impl Executor {
    fn run_all(&mut self, futures: Vec<Future>) -> Vec<(usize, Result<Future::Item, Future::Error>)> {
        let mut done = 0;
        let mut results = Vec::with_capacity(futures.len());
        while done != futures.len() {
            for (i, f) in futures.iter_mut().enumerate() {
                match f.poll() {
                    Ok(Async::Ready(t)) => {
                        // done
                        results.push((i, Ok(t)));
                    }
                    Ok(Async::NotReady) => {
                        // poll again
                        continue;
                    }
                    Err(e) => {
                        results.push((i, Err(e)));
                        done += 1
                    }
                }
            }
        }

        results
    }
}
*/
