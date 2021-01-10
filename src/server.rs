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

    fn read_username_buffer(&mut self, mut stream: &TcpStream) -> Option<Vec<u8>> {
        let mut buffer = vec![0; 32 as usize];
        return match stream.read(&mut buffer) {
            Ok(n) => {
                return Some(
                    (String::from_utf8_lossy(&buffer[..n]).trim().to_owned() + ": ").into_bytes(),
                );
            }
            Err(_) => {
                println!(
                    "Failed to read username, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                None
            }
        };
    }

    fn broadcast(&mut self, message: Vec<u8>) {
        let clients = self.clients.write().unwrap();
        for (_, mut s) in &*clients {
            s.write(&message).unwrap();
            s.flush().unwrap();
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream) {
        let username_buffer = self.read_username_buffer(&stream).unwrap();
        self.clients.write().unwrap().insert(
            username_buffer.to_owned().to_vec(),
            stream.try_clone().unwrap(),
        );

        let mut buffer = vec![0; 512 as usize];
        while match stream.read(&mut buffer) {
            Ok(0) => {
                let mut msg = username_buffer.clone();
                msg.extend("has left the chat.".as_bytes().iter().cloned());
                self.broadcast(msg);
                stream.shutdown(Shutdown::Both).unwrap();
                true
            }
            Ok(n) => {
                let mut msg = username_buffer.clone();
                msg.extend(buffer[..n].iter().cloned());
                self.broadcast(msg);
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
}
