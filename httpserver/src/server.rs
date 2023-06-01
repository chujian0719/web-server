use http::httprequest::HttpRequest;
use std::net::TcpListener;
use std::io::prelude::*;

pub struct Server<'a> {
  socket_addr: &'a str,
}

impl<'a> Server<'a> {
  pub fn new(socket_addr: &'a str) -> Self {
    Server {
      socket_addr
    }
  }

  pub fn run(&self) {
    let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
    for stream in connection_listener.incoming() {
      let mut _stream = stream.unwrap();
      println!("Connection established!");
      let mut read_buffer = [0; 1024];
      _stream.read(&mut read_buffer).unwrap();
      let req:HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
      println!("{:?}", req);
    }
  }
}