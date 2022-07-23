use std::io::Read;
use std::net::{TcpListener, TcpStream};

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

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 2048];
                    stream.read(&mut buffer);
                }
                Err(error) => {
                    println!("Connection failed : {}", error);
                }
            }
        }
    }
}
