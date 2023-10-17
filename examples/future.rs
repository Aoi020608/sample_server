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
    fn run_all<F>(&mut self, futures: Vec<F>) -> Vec<(usize, Result<F::Item, F::Error>)> where F: Future {
        let mut done = 0;
        let mut results = Vec::with_capacity(futures.len());
        let mut tasks = Vec::new();
        for _  in 0..futures.len() {
            tasks.push(Task::new());
        }

        while done != futures.len() {
            for (i, f) in futures.iter_mut().enumerate() {
                // don't poll futures that can't make progress
                if !tasks[i].notified() {
                    continue;
                }

                task::set_current(tasks[i].clone());
                match f.poll() {
                    Ok(Async::Ready(t)) => {
                        // done
                        results.push((i, Ok(t)));
                    }
                    Err(e) => {
                        results.push((i, Err(e)));
                        done += 1
                    }
                    Ok(Async::NotReady) => {
                        // f *must* have arranged for tasks[i] (its task) to be notified later
                        continue;
                    }
                }
            }

            // wait for Task::notify to be called
        }

        results
    }
}
*/

/*
struct Foo {
    // But with O_NONBLOCKING set
    fd: std::net::TcpStream,
}
*/

/*
impl Future for Foo {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.fd.read() {
            Ok(r) => {
                eprintln!("got {} bytes", r.len());
            }
            Err(io::Error::WouldBlock) => {
                // do something to make sure we are waken up
                let reacotor = Handle::current();
                reactor.register(self.fd, Operation::Read, task::current());
                return Ok(Async::NotReady);
            }
            Err(io::Error::Closed) => {
                return Ok(Async::Ready);
            }
            Err(e) => {
                return Err(e);
            }

        }

    }
}
*/

/*
enum Operaiton {
    Read,
    Write,
}
*/

/*
fn reactor_thread(notify_me: mpsc::Receiver<(Task, FD, Operation)> {
    let waiting_for: HashMap<(FD, Operation), Task>;

    loop {
        // accept new things to watch for
        while let Some((task, fd, op)) = notify_me.try_recv() {
            waiting_for.insert((fd, op), task);
        }

        let select = waiting_for.keys().collect();
        for (fd, op) in epoll(select) {
            waiting_for.remove((fd, op)).notify();
        }
    }
}
*/
