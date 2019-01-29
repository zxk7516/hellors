#![allow(unused_imports)]
use tokio::fs;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|_socket| {
            println!("{:?}", _socket);
            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });

    println!("server running on localhost:6142");

    tokio::run(server);
}
