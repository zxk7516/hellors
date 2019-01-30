#![allow(warnings)]
#![feature(mpsc_select)]

use crossbeam_channel::bounded;
use crossbeam_utils::thread;

fn main() {
    let people = vec!["Anna", "Bob", "Cody", "Dave", "Eva"];
    let (s, r) = bounded(1);

    let seek = |name, s, r| {
        select! {
            recv(r) -> peer => println!("{} received a message from", name, peer.unwrap()),
            send(s,name) -> _ =>{},
        }
    };
}
