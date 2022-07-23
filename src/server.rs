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

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Read data -> {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(error) => {
                            println!("Reading data failed -> {:}", error)
                        }
                    };
                }
                Err(error) => {
                    println!("Connection failed : {}", error);
                }
            }
        }
    }
}
