use std::{net::TcpListener, io::Read};
use super::router::Router;
use http::httprequest::HttpRequest;
use std::str;

pub struct Server<'a>{
    socket_addr:&'a str,
}

impl <'a> Server<'a>{
    pub fn new(socket_addr:&'a str)->Server<'a>{
        Server { socket_addr }
    }
    pub fn run(&self){
        let addr = self.socket_addr;
        println!("{}",addr);
        let connection_listener = TcpListener::bind(addr).unwrap();
        println!("port {} connected",addr);
        for stream in connection_listener.incoming(){
            let mut stream = stream.unwrap();
            let mut buffer = [0;512];
            stream.read(&mut buffer).unwrap();
            let req:HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
            Router::router(req,&mut stream);
        }

    }
}