use std::net::UdpSocket;

mod flasch;
mod img;

fn main() {

let host = "Localhost";
let w: u64 = 25;
let h: u64 = 30;

let flasch = flasch::flaschen::new(host, w, h);

println!("flasch: {:?}",flasch);
}
