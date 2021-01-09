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

            stream.write(&username.trim().as_bytes()).unwrap();

            let mut c_stream = stream.try_clone().unwrap();
            thread::spawn(move || loop {
                let mut msg = String::new();
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut msg)
                    .expect("Failed to read line");
                c_stream.write(&msg.trim().as_bytes()).unwrap();
            });

            loop {
                let mut buff = vec![0; 512 as usize];
                match stream.read(&mut buff) {
                    Ok(n) => {
                        // println!("usize {:?}", n);s
                        println!("{}", String::from_utf8_lossy(&buff[..n]).trim());
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
