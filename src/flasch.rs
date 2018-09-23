use std::net::{SocketAddr, UdpSocket};

#[derive(Debug)]
pub struct flaschen <'a> {
	fd: &'a str,
	socket: UdpSocket,
	height: u64,
	width: u64,
}

impl <'a>flaschen<'a>{
	pub fn new(host: &str, w: u64, h: u64) -> flaschen {
		//create a list of address to bind the socket to 
		let addrs = [
			SocketAddr::from(([127,0,0,1], 3400)),
			SocketAddr::from(([127,0,0,1], 3401)),
			SocketAddr::from(([127,0,0,1], 3402)),
			SocketAddr::from(([127,0,0,1], 3403)),
		];

		flaschen {
			fd: host,
			socket: UdpSocket::bind(&addrs[..]).expect("Can not bind socket"),
			width: w,
			height: h,
		}
	}
}

