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
    pub fn run(self) {
        println!("Listening on {} ....", self.addr);
    }
}