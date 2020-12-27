use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone, Debug)]
pub struct Server {
    clients: Arc<RwLock<Vec<TcpStream>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream) {
        let mut buffer = vec![0; 32 as usize];
        while match stream.read(&mut buffer) {
            Ok(_) => {
                let clients = self.clients.write().unwrap();
                for mut s in &*clients {
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
                            self_clone
                                .clients
                                .write()
                                .unwrap()
                                .push(stream.try_clone().unwrap());
                            self_clone.handle_client(stream);
                            // connection succeeded
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
