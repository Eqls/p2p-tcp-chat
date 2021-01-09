use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;
use std::{
    collections::HashMap,
    io::{Read, Write},
    iter,
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
            Ok(n) => {
                self.clients.write().unwrap().insert(
                    username_buffer[..n - 1].to_owned().to_vec(),
                    stream.try_clone().unwrap(),
                );
            }
            Err(_) => {
                println!(
                    "Failed to read username, tserminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
            }
        };
        let mut buffer = vec![0; 512 as usize];
        while match stream.read(&mut buffer) {
            Ok(n) => {
                let clients = self.clients.write().unwrap();
                for (_, mut s) in &*clients {
                    let mut msg = (String::from_utf8_lossy(&username_buffer[..n - 1])
                        .trim()
                        .to_owned()
                        + ": ")
                        .into_bytes();
                    msg.extend(buffer[..n].iter().cloned());
                    println!("DEBUG: buffer size is {:?} bytes", &buffer.len());
                    println!("DEBUG: received msg size is {:?} bytes", n);
                    println!("DEBUG: sent message size is {:?}", &msg.len());
                    println!("DEBUG: {:?}", String::from_utf8_lossy(&msg).trim());
                    println!("DEBUG: {:?}", String::from_utf8_lossy(&buffer[..n]).trim());
                    s.write(&msg).unwrap();
                    s.flush().unwrap();
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
