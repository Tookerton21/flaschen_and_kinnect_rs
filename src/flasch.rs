use std::net::{SocketAddr, UdpSocket};


#[derive(Debug)]
pub struct Flaschen <'a> {
	fd: &'a str,
	socket: UdpSocket,
	height: u64,
	width: u64,
}

impl <'a>Flaschen<'a>{
	pub fn new(host: &str, h: u64, w: u64) -> Flaschen {
		//create a list of address to bind the socket to, to be able to send the data from. 
		let addrs = [
			SocketAddr::from(([127,0,0,1], 3400)),
			SocketAddr::from(([127,0,0,1], 3401)),
			SocketAddr::from(([127,0,0,1], 3402)),
			SocketAddr::from(([127,0,0,1], 3403)),
		];

		Flaschen {
			fd: host,
			socket: UdpSocket::bind(&addrs[..]).expect("Can not bind socket"),
			width: w,
			height: h,
		}
	}

	pub fn send(&self, buf: Vec<u8>) -> usize {
		self.socket.connect("127.0.0.1:1337").expect("Connection Failed");
		let res = self.socket.send(&buf[..]).expect("could not send message");
		res

	}

}

/*	
//
//						TESTS
*/

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	pub fn test_send(){

	}
}

