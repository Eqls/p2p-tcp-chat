use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;
use std::{
    collections::HashMap,
    io::{Read, Write},
};

#[derive(Clone, Debug)]
pub struct Server {
    clients: Arc<RwLock<HashMap<Vec<u8>, TcpStream>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream) {
        let mut username_buffer = vec![0; 32 as usize];
        match stream.read(&mut username_buffer) {
            Ok(_) => {
                self.clients
                    .write()
                    .unwrap()
                    .insert((*username_buffer).to_vec(), stream.try_clone().unwrap());
            }
            Err(_) => {
                println!(
                    "Failed to read username, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
            }
        };
        let mut buffer = vec![0; 32 as usize];
        while match stream.read(&mut buffer) {
            Ok(_) => {
                let clients = self.clients.write().unwrap();
                for (_, mut s) in &*clients {
                    s.write(&username_buffer).unwrap();
                    s.write(&buffer).unwrap();
                }
                true
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
    }

    pub fn run(&self) -> JoinHandle<()> {
        let self_clone = self.clone();
        thread::spawn(move || {
            let self_clone = self_clone.clone();
            let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
            println!("Server listening on port 3333");
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let mut self_clone = self_clone.clone();
                        thread::spawn(move || {
                            self_clone.handle_client(stream);
                        });
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        /* connection failed */
                    }
                }
            }
            drop(listener);
        })
    }
}
