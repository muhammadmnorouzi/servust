use crate::http::{HttpStatusCode, ParsingError, Request, Response};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait HttpRequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, error: &ParsingError) -> Response {
        println!("Failed to parse request with error: {}", error);
        Response::new(HttpStatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { address: addr }
    }

    pub fn run(self, mut handler: impl HttpRequestHandler) {
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
                                Ok(request) => handler.handle_request(&request),
                                Err(error) => handler.handle_bad_request(&error),
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
