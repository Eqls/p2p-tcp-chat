use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;
use std::{
    collections::HashMap,
    io::{Read, Write},
};
#[derive(Debug)]
struct Client {
    username: String,
    stream: TcpStream,
}

#[derive(Clone, Debug)]
pub struct Server {
    clients: Arc<RwLock<HashMap<usize, Client>>>,
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
            let mut handles: Vec<JoinHandle<()>> = Vec::new();
            let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
            println!("Server listening on port 3333");
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let mut self_clone = self_clone.clone();
                        let mut id = 0;

                        {
                            let clients = &self_clone.clients.write().unwrap();

                            if clients.len() > 0 {
                                id = clients.len();
                            }
                        }

                        let handle = thread::spawn(move || {
                            {
                                let mut clients = self_clone.clients.write().unwrap();
                                clients.insert(
                                    id.clone(),
                                    Client {
                                        username: String::new(),
                                        stream: stream.try_clone().unwrap(),
                                    },
                                );
                            }

                            self_clone.handle_client(stream.try_clone().unwrap(), &id);
                        });
                        handles.push(handle);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        /* connection failed */
                    }
                }
            }

            for handle in handles {
                handle.join().unwrap();
            }
            drop(listener);
        })
    }

    fn read_username_buffer(&mut self, mut stream: &TcpStream) -> Option<String> {
        let mut buffer = vec![0; 32 as usize];
        return match stream.read(&mut buffer) {
            Ok(n) => {
                return Some(String::from_utf8_lossy(&buffer[..n]).trim().to_owned());
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

    fn disconnect(&mut self, id: &usize) {
        self.clients.write().unwrap().remove(id);
        println!("{:?}", self.clients.read().unwrap());
    }

    fn broadcast(&mut self, message: Vec<u8>) {
        let mut clients = self.clients.write().unwrap();
        for client in clients.values_mut() {
            client.stream.write(&message).unwrap();
            client.stream.flush().unwrap();
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream, id: &usize) {
        let username = self.read_username_buffer(&stream).unwrap();
        {
            self.clients.write().unwrap().get_mut(&id).unwrap().username = username.clone();
        }

        let mut buffer = vec![0; 512 as usize];
        while match stream.read(&mut buffer) {
            Ok(0) => {
                let mut msg = username.clone().as_bytes().to_vec();
                msg.extend("has left the chat.".as_bytes().iter().cloned());
                self.broadcast(msg);
                self.disconnect(id);
                false
            }
            Ok(n) => {
                let mut msg = username.clone().as_bytes().to_vec();
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
