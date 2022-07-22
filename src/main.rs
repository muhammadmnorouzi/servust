fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    address: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { address: addr }
    }

    fn run(self) {}
}

struct HttpRequest {
    path: String,
    query_string: String,
    method: HttpMethod,
}

enum HttpMethod {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
