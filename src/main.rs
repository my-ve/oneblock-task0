//Bring std::io::prelude into scope to get access to certain traits that let us read from and write to the stream.
use std::io::prelude::*;
//Using TcpListener to listen for TCP connections;
//Using TcpStream to read data from TCP connections.
use std::net::{TcpListener, TcpStream};

fn main() {
    //Creates a new TcpListener which will be bound to the any address and port 8888.
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();

    //Iterating over the connections being received on this listener.
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

//Create a function to handle the request from the TCP connection,
//and make the stream parameter mutable.
fn handle_connection(mut stream: TcpStream) {
    //Declare a buffer on the stack to hold the data that is read in.
    //FIXME : Avoid using fixed size buffer.
    let mut buffer = [0; 1024];

    //Pull some bytes into the buffer, and we need to know how many bytes were read.
    let bytes = match stream.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(e) => panic!("Problem reading from tcpstream: {:?}!", e),
    };

    //Print the the data being sent from the TCP client.
    println!("Request: {}[END]", String::from_utf8_lossy(&buffer[..]));

    //Write back the received data to the TCP connection.
    stream.write(&buffer[..bytes]).unwrap();

    //flush will wait and prevent the program from continuing
    //until all the bytes are written to the connection.
    stream.flush().unwrap();
}
