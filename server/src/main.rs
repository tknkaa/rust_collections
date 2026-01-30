use core::time;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream}, 
};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
   }
}

fn handle_connection(mut stream: TcpStream) {
        thread::sleep(time::Duration::from_secs(3));
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!\n";
        stream.write_all(response.as_bytes()).unwrap();
}
