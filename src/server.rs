use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        // return new instances
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok(_) => {
                    let a = 5;
                    println!("OK")
                },
                Err(e) => println!("Faild t oestablish a connection: {}", e),
            }
        }
    }
}
