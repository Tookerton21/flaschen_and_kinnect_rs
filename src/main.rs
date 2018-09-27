use std::net::UdpSocket;
use std::env;

mod flasch;
mod img;

use img::Img;

//use img::img;
fn main() {

//let path = env::current_dir().unwrap();
		let h: u64 = 30;
		let w: u64 = 30;
		let z: u64 = 1; 
		let img_path = "src/black-tick.jpg";

		//create new flaschen taschen 
		let fl = flasch::Flaschen::new("Localhost", h, w);

		//Create a new img
		let mut i = img::Img::new(h, w, z);

		//open the image
		let res = i.open_image(img_path); 
		assert!(res); //panic if false

		println!("flaschen: {:?}", fl);
		println!("img: {:?}", i);
		println!("bin img: {:?}", i.binary_img());
		let ret = fl.send(i.binary_img());
		//let tester: Vec<u8> = vec![1,2,3];
		//let ret = fl.send(tester);

		println!("ret: {:?}", ret);
}

#[cfg(test)]
mod tests{
	use super::*;

	//Test to ensure that the that one can send an image to display on 
	//the flaschen taschen server, and server must be running during the 
	//test!!
	#[test]
	pub fn test_single_img(){
		let h: u64 = 20;
		let w: u64 = 20;
		let z: u64 = 0; 
		let img_path = "images/front_display.jpg";

		//create new flaschen taschen 
		let fl = flasch::Flaschen::new("Localhost", h, w);

		//Create a new img
		let mut i = img::Img::new(h, w, z);


		let res = i.open_image(img_path); 
		assert_eq!(true, res); //panic if false

		fl.send(i.binary_img());


	}
}
