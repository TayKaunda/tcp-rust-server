// importing necessary modules from rust libraries
use std::io:: {Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_clients(mut stream: TcpStream) {
    //this is a buffer to read client data
    let mut buffer = [0; 1024];
    //this line reads from stream and stores in buffer.
    stream.read(&mut buffer).expect("Failed to read from client!");
    // this line converts buffwer data into a utf-8 string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Recieved request: {}", request);
    let response = "Hello, Client!".as_bytes();
stream.write(response).expect("Failed to write to client!");
}

//Entry pont
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Listening on port 127.0.0.1:8080");
    
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                std::thread::spawn(||  handle_clients(stream));

            }
            Err(e) => {
             eprintln!("Failed to establish connection: {}", e);  
             // stderr- standerd error stream 
            }
        }
    }
}
