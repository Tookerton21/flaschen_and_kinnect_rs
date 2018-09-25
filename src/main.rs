use std::net::UdpSocket;
use std::env;

mod flasch;
mod img;

//use img::img;
fn main() {

let path = env::current_dir().unwrap();
println!("The current directory is {}", path.display());
let host = "Localhost";
let w: u64 = 25;
let h: u64 = 30;

let flasch = flasch::flaschen::new(host, w, h);
let mut i = img::img::new(0 as u64,0 as u64,1 as u64);
let res = i.open_image("src/test_pic.jpg");



//println!("flasch: {:?}",flasch);
}
