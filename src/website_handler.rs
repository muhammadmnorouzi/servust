use crate::{
    http::{HttpStatusCode, Response},
    server::HttpRequestHandler,
};

pub struct WebsiteHandler;

impl HttpRequestHandler for WebsiteHandler {
    fn handle_request(&mut self, _request: &crate::http::Request) -> crate::http::Response {
        Response::new(
            HttpStatusCode::Ok,
            Some("<title>This is the title</title><h1>This is a test</h1>".to_string()),
        )
    }

    // fn handle_bad_request(&mut self, error: &crate::http::ParsingError) -> crate::http::Response {
    //     println!("Failed to parse request with error: {}", error);
    //     crate::http::Response::new(crate::http::HttpStatusCode::BadRequest, None)
    // }
}
