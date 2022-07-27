use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
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

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 4096];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Read data -> {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(error) => println!("Parsing request failed -> {}", error),
                            }
                        }
                        Err(error) => println!("Reading data failed -> {}", error),
                    };
                }
                Err(error) => println!("Connection failed : {}", error),
            }
        }
    }
}
