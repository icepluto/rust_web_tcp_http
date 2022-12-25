use std::{net::TcpListener, io::{Read, Write}};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        let mut buffer = [0;1024];

        stream.read(&mut buffer).unwrap();
        let s = String::from_utf8_lossy(&buffer);
        println!("buffer:{:?}",s);
        stream.write(&mut buffer).unwrap();
    }
}