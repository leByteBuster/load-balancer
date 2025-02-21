use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Processor {
    pub servers: Arc<Mutex<Vec<String>>>,
    pub last_request: Arc<Mutex<usize>>,
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            servers: Arc::new(Mutex::new(vec![])),
            last_request: Arc::new(Mutex::new(0)),
        }
    }

    pub fn register_server(&mut self, server: String) {
        self.servers.lock().unwrap().push(server);
    }

    pub fn register_servers(&mut self, to_register: &mut Vec<String>) {
        self.servers.lock().unwrap().append(to_register);
    }
}
