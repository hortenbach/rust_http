use std::io::{ Write, Read };
use std::convert::TryFrom;
use super::http::ParseError;
use crate::http::{ Request, Response, StatusCode };
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    // can be any name but constructor is commonly expected to
    // be called 'new'
    // Uppercase Self in a struct is an alias for the struct name
    // meaning in this case 'Self' and 'Struct' are interchangable
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }

    // 'self' follow regular ownership rules which means 'run'
    // takes ownership of the entire struct. This also means
    // our struct will be deallocated at the end of the function.
    // If we dont want that we can make run a reference.
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {} ....", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Connected.");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            response.send(&mut stream);
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}