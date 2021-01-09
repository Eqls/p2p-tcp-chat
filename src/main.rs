use std::io;
use std::string::String;

pub mod client;
pub mod packet;
pub mod server;
use server::Server;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Action {
    JOIN,
    CREATE,
    NONE,
}

fn main() {
    let username = get_username();

    match join_create() {
        Action::CREATE => {
            let server = Server::new();
            server.run();
            client::join(username);
        }
        Action::JOIN => client::join(username),
        _ => println!("Nothing."),
    }
}

fn get_username() -> String {
    let mut username = String::new();

    println!("Please enter your username:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    let username = username.trim().to_string();

    username
}

fn join_create() -> Action {
    println!("Enter /join to join a room, /create to create one.");

    let mut action_type = String::new();

    io::stdin()
        .read_line(&mut action_type)
        .expect("Failed to read line");

    return match action_type.as_str().trim() {
        "/join" => Action::JOIN,
        "/create" => Action::CREATE,
        _ => Action::NONE,
    };
}
