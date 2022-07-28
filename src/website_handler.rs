use std::fs;

use crate::{
    http::{HttpStatusCode, Response},
    server::HttpRequestHandler,
};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    // Unsafe in case of potentially directory traversal attacks
    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }
}

impl HttpRequestHandler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        match request.method() {
            crate::http::Method::GET => match request.path() {
                "/" => Response::new(HttpStatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(HttpStatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(HttpStatusCode::Ok, Some(content)),
                    None => {
                        Response::new(HttpStatusCode::NotFound, self.read_file("not_found.html"))
                    }
                },
            },
            crate::http::Method::DELETE => todo!(),
            crate::http::Method::POST => todo!(),
            crate::http::Method::PUT => todo!(),
            crate::http::Method::HEAD => todo!(),
            crate::http::Method::CONNECT => todo!(),
            crate::http::Method::OPTIONS => todo!(),
            crate::http::Method::TRACE => todo!(),
            crate::http::Method::PATCH => todo!(),
        }
    }

    // fn handle_bad_request(&mut self, error: &crate::http::ParsingError) -> crate::http::Response {
    //     println!("Failed to parse request with error: {}", error);
    //     crate::http::Response::new(crate::http::HttpStatusCode::BadRequest, None)
    // }
}
