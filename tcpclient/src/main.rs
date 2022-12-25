use std::{net::TcpStream, io::{Write, Read}};
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.write("hello rust".as_bytes()).unwrap();

    let mut buffer = [0;128];
    stream.read(&mut buffer).unwrap();
    println!("{}",std::str::from_utf8(&mut buffer).unwrap());
}
