use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::String;
use std::thread;

// use crate::packet::Packet;

pub fn join(username: String) {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            stream.write(&username.clone().into_bytes());

            let mut c_stream = stream.try_clone().unwrap();
            thread::spawn(move || loop {
                let mut msg = String::new();
                io::stdin()
                    .read_line(&mut msg)
                    .expect("Failed to read line");
                c_stream.write(&msg.into_bytes());
            });

            loop {
                let mut buff = vec![0; 512 as usize];
                match stream.read(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        println!("message recv {:?}", String::from_utf8_lossy(&msg).trim());
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
