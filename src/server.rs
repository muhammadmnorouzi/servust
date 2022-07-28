use crate::http::{HttpStatusCode, Request, Response};
use std::convert::TryFrom;
use std::io::{Read, Write};
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

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // dbg!(request);
                                    Response::new(
                                        HttpStatusCode::Ok,
                                        Some("<h1>This is working</h1>".to_string()),
                                    )
                                }
                                Err(error) => {
                                    println!("Parsing request failed -> {}", error);
                                    Response::new(HttpStatusCode::Ok, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response with error : {}", e);
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
