//The img struct is to create a raw PPM file that can be used to take any

extern crate image;

use std::path::Path;

const SIZE: (u64, u64) = (25,35);

#[derive(Debug, Clone)]
pub struct img {
	header:  String,
	rgb_data: Vec<u8>,
}

	//extern crate image;

impl img{
	//Creates a new PPM file, with the required header for the flaschen taschen project, 
	//and an empty vector to hold the binary information from the rgb data of the image.
	pub fn new(mut height: u64, mut width: u64, z_layer: u64) -> img{
		if height <= 0 || width <= 0 {
			height = SIZE.0;
			width = SIZE.1;
		}

		let h = format!("P6\n{h} {w}\n#FT: {h} {w} {z}\n255", h=height, w=width, z=z_layer);
		img {
			header : h,
			rgb_data: Vec::new(),
		}
	}


	//Taken the address of the image and open it using the image crate. Get the raw
	//Rgb data from the Dynamicimage. Save this Vec<u8> into the image structure. Returns
	//true if the image was able to open and false is unable to open the image.
	pub fn open_image(& mut self, img_file: &str) ->  bool {
		let mut img = image::open(&Path::new(img_file));//.expect("Opening image failed");
		
		let res = match img {
			Ok(i)	=>	{println!("its ok"); self.rgb_data = i.to_rgb().into_raw(); true},
			Err(e)	=> 	{println!("Error: {:?}", e); false},
		};
		//self.rgb_data = img.to_rgb().into_raw();
		res
	}

	//Return the raw PPM data into a since binary vec to be sent as a udp packet
	pub fn binary_img(&self) -> Vec<u8> {
		let mut h = self.header.clone().into_bytes();
		h.append(&mut self.rgb_data.clone());
		h
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	//#[should_panic (expected = "File could not be opened!")]
	pub fn test_image(){
		let mut img = img::new(32, 32, 1);
		let mut test = format!("P6\n{h} {w}\n#FT: {h} {w} {z}\n255", h=32, w=32, z=1);
		assert!(test.eq_ignore_ascii_case(img.header.as_str()));

		img = img::new(0,0,1);
		test = format!("P6\n{h} {w}\n#FT: {h} {w} {z}\n255", h=25, w=35, z=1);

		assert!(test.eq_ignore_ascii_case(img.header.as_str()));

		//Test that the image truely does get opened
		//assert!(img.open_image("/images/front_display.jpg"));
		//img.open_image("doesnt_exit.jpg"); //This should panic
	}

	#[test]
	pub fn test_image_open(){
		let mut img = img::new(32, 32, 1);
		//use std::path::Path;

		//let path = Path::new("foo.txt");
		assert_eq!(true, img.open_image("src/test_pic.jpg"));
	}
}