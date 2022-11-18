fn main() {
    server = Server::new("127.0.0.1:8080");
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    // can be any name but constructor is commonly expected to
    // be called 'new'
    fn new(addr: String) -> Server {
        Server {
            addr
        }
    }
    
    // 'self' follow regular ownership rules which means 'run'
    // takes ownership of the entire struct
    fn run(self) {

    }
}