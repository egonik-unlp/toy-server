use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:2190").unwrap();
    let mut buff = Vec::with_capacity(4096);
    let req = " / HTTP/1.1\r\n\
    Host: localhost\r\n\
    \r\n";
    stream.write(req.as_bytes()).unwrap();
    stream.read_to_end(&mut buff).expect("PUMBA");
    println!("{}", String::from_utf8(buff).unwrap())
}
