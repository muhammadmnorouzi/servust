use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { address: addr }
    }

    pub fn run(self) {
        println!("listening on {:}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
    }
}
