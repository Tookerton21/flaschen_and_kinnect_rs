use std::net::{SocketAddr, UdpSocket, ToSocketAddrs, Ipv4Addr};


//Change this to the IP address of that of your machine on the networl
//that you are working on. If dynamic change to static.
const IP: [u8; 4]	= [192, 168, 0, 104];

#[derive(Debug)]
pub struct Flaschen  {
	fd: String,
	socket: UdpSocket,
	height: u64,
	width: u64,
}

impl Flaschen{
	pub fn new(host: &str, h: u64, w: u64) -> Flaschen {
		
		//Create an Ip-v4 of the host 
		let ip_v4 = Ipv4Addr::new(IP[0], IP[1], IP[2], IP[3]);


		//create a list of address to bind the socket to, to be able to send the data from. 
		let addrs = [
			SocketAddr::from((ip_v4, 3411)),
			SocketAddr::from((ip_v4, 3412)),
			SocketAddr::from((ip_v4, 3413)),
			SocketAddr::from((ip_v4, 3414)),
			SocketAddr::from((ip_v4, 3415)),
			SocketAddr::from((ip_v4, 3416)),
			SocketAddr::from((ip_v4, 3417)),
			SocketAddr::from((ip_v4, 3418)),
			SocketAddr::from((ip_v4, 3419)),
			SocketAddr::from((ip_v4, 3420)),
			SocketAddr::from((ip_v4, 3421)),
		];

		let mut host_and_port: String = host.to_string();
		host_and_port.push_str(":1337"); //Server is on port 1337
		let fd: String;

		//If the host is localhost
		if host == "localhost"{
			fd = "127.0.0.1:1337".to_string();
		}
		//Look for the host name if it exists
		else {

			let addrs_iter = host_and_port.to_socket_addrs();

			fd = match addrs_iter {
				Ok(mut addr)	=> { addr.next().unwrap().to_string()},
				Err(_)	=> {println!("Host Not found, setting to localhost"); "127.0.0.1:1337".to_string()},
			};
		}
		
		Flaschen {
			fd: fd,
			socket: UdpSocket::bind(&addrs[..]).expect("Can not bind socket"),
			width: w,
			height: h,
		}
	}

	pub fn send(&self, buf: Vec<u8>) -> usize {
		
		self.socket.connect(self.fd.clone()).expect("Connection Failed!!!");
		let res = self.socket.send(&buf[..]).expect("could not send message!");
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

	//Testing the functionality of the machine_ip to see if it is matching up for the 
	//Needs that I want to get the IP of the computer on the current network.
	#[test]
	pub fn test_machine_ip(){
		let fl = Flaschen::new("localhost", 20, 20);

		assert_eq!(fl.fd, "127.0.0.1:1337");
	}
}

