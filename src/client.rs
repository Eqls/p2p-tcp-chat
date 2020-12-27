use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::String;
use std::thread;

pub fn join() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let mut c_stream = stream.try_clone().unwrap();
            thread::spawn(move || loop {
                let mut msg = String::new();
                io::stdin()
                    .read_line(&mut msg)
                    .expect("Failed to read line");
                c_stream.write(&msg.into_bytes());
                println!("Sent Hello, awaiting reply...");
            });

            loop {
                let mut buff = vec![0; 32 as usize];
                match stream.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        println!("message recv {:?}", String::from_utf8_lossy(&msg));
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
