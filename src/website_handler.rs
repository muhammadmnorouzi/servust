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
}

impl HttpRequestHandler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        match request.method() {
            crate::http::Method::GET => match request.path() {
                "/" => Response::new(
                    HttpStatusCode::Ok,
                    Some("<h1>Welcome to root page</h1>".to_string()),
                ),
                "/hello" => Response::new(
                    HttpStatusCode::Ok,
                    Some("<h1>Welcome to Hello page</h1>".to_string()),
                ),
                _ => Response::new(
                    HttpStatusCode::NotFound,
                    Some("<h1>Page not found</h1>".to_string()),
                ),
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
