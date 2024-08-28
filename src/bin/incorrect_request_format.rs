use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    let mut buff = Vec::with_capacity(4096);
    let malformed_request = "GET / HTTP/1.1";
    stream.set_read_timeout(None).unwrap();
    stream.write(malformed_request.as_bytes()).unwrap();
    stream.set_write_timeout(None).unwrap();

    stream.read(&mut buff).unwrap();
    println!("{}", String::from_utf8(buff).unwrap())
}
