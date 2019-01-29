#![allow(unused_imports)]
use tokio::fs;
use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket: TcpStream| {
            let (reader, writer) = socket.split();
            let amount = io::copy(reader, writer);

            let msg = amount.then(|result| {
                match result {
                    Ok((amount, _, _)) => println!("wrote {} bytes", amount),
                    Err(e) => println!("error: {}", e),
                }

                Ok(())
            });

            tokio::spawn(msg);
            Ok(())
        })
        .map_err(|_err| ());

    println!("server running on localhost:6142");

    tokio::run(server);
}
