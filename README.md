# p2p-tcp-chat
A simple TCP chat written in Rust, using threads for each client.

# todo
 * Each client ID saving.
 * Passing server ownership to other connected client if host disconnects.
 * Displaying username next to a message.
 * Server command system (`/users` etc.)
 * Possibly event based system instead of threads mutating state.
