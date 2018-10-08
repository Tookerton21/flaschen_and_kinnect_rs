//The img struct is to create a raw PPM file that can be used to take any

extern crate image;

use self::image::{GenericImage};
use std::path::Path;

//Default setting for Display width and size
const SIZE: (u64, u64) = (25,35);
const DISPLAY: (u32, u32) = (640, 480); //(wxh)
#[derive(Debug, Clone)]
pub struct Img {
	height: u64,
	width: u64,
	header:  String,
	rgb_data: Vec<u8>,
}

impl Img{
	//Creates a new PPM file, with the required header for the flaschen taschen project, 
	//and an empty vector to hold the binary information from the rgb data of the image.
	pub fn new(mut height: u64, mut width: u64, z_layer: u64) -> Img{
		if height <= 0 || width <= 0 {
			height = SIZE.0;
			width = SIZE.1;
		}

		let h = format!("P6\n{h} {w}\n#FT: 0 0 {z}\n255\n", h=height, w=width, z=z_layer);
		Img {
			height: height,
			width: width,
			header : h,
			rgb_data: Vec::new(),
		}
	}

	//Takes a raw depth data array and turn the distance point into abn rgb dynamic image
	//, resizes it and then copys the rgb data to the image struct. 
	pub fn convert_data_img(&mut self, data: &[u16]){
		let mut img: image::DynamicImage = image::DynamicImage::new_rgb8(DISPLAY.0, DISPLAY.1);
		let mut pos = 0;
		
		for x in 0..DISPLAY.0 {
			for y in 0..DISPLAY.1 {
				if pos > data.len(){
					panic!("Outside of bounds!!");
				}
				let pix = (data[pos] % 255) as u8;
				img.put_pixel(x,y, image::Rgba([pix, pix, pix, 255]));
				pos += 1;
			}
		}

		//resize the image to fit on the flaschen display
		self.rgb_data = img.resize(self.width as u32, self.height as u32, image::FilterType::Nearest).to_rgb().into_raw();
		
	}

	pub fn get_img(&mut self, data: &[u8]) {
		let mut img: image::DynamicImage = image::DynamicImage::new_rgb8(DISPLAY.0, DISPLAY.1);
		let mut pos = 0;
		let inc = 3;

		for x in 0..DISPLAY.0 {
			for y in 0..DISPLAY.1 {
				if pos > data.len() {
					panic!("Outside of bounds!!");
				}
				img.put_pixel(x,y, image::Rgba([data[pos], data[pos+1], data[pos+2], 255]));
				pos = pos+inc;
			}
		}

		self.rgb_data = img.resize(self.width as u32, self.height as u32, image::FilterType::Nearest).to_rgb().into_raw();

	}
	//Taken the address of the image and open it using the image crate. Get the raw
	//Rgb data from the Dynamicimage after resizing it to fit the flaschen display.
	//Save this Vec<u8> into the image structure. Returns true if the image was able 
	//to open and false is unable to open the image.
	pub fn open_image(& mut self, img_file: &str) ->  bool {
		let img = image::open(&Path::new(img_file));
		let res = match img {
			Ok(i)	=>	{ 	self.rgb_data = i.resize(self.width as u32, self.height as u32, image::FilterType::Nearest)
										     .to_rgb()
										     .into_raw();
							true	
						},
			Err(_)	=> 	false,
		};
		res
	}

	//Return the raw PPM data into a since binary vec to be sent as a udp packet
	pub fn binary_img(&self) -> Vec<u8> {
		let h = self.rgb_data.clone();
		h
	}

	pub fn clear_data(&mut self) {
		self.rgb_data = Vec::new();
	}
}

/*
//The kinect V1, which is the version that This application is using  contains 
//rw depth values between 0-2048. For kinect v2 it has a range between 4500. Takes in 
//raw data points and converts this to Rgb data.
pub fn convert_to_rgb(data: &[u16]) -> Vec<u8>{

	let mut rgb_data:Vec<u8> = Vec::new();
	
	//iterate through list take the mod of the depth data to ensure that it is an 
	//rgb color and add this into a vector.
	for elem in 0..data.len(){
		rgb_data.push((data[elem] % 255) as u8);
	}
	rgb_data
}
*/

/*	
//
//						TESTS
*/

#[cfg(test)]
mod tests{
	use super::*;

	//Test that the header is being set correctly when passed valid parameters
	//and that default values kick in if they are not for the header.
	#[test]
	pub fn test_image(){
		let mut img = Img::new(32, 32, 1);
		let mut test = format!("P6\n{h} {w}\n#FT: 0 0 {z}\n255\n", h=32, w=32, z=1);
		assert!(test.eq_ignore_ascii_case(img.header.as_str()));

		img = Img::new(0,0,1);
		test = format!("P6\n{h} {w}\n#FT: 0 0 {z}\n255\n", h=25, w=35, z=1);

		assert!(test.eq_ignore_ascii_case(img.header.as_str()));
	}

	//Test that opening a valid image returns true, and that if one opens
	//an invalid image the function returns false.
	#[test]
	pub fn test_image_open(){
		let mut img = Img::new(20, 20, 0);
		assert_eq!(true, img.open_image("src/test_pic.jpg"));

		//assert_eq!(false, img.open_image("does_not_exist.jpg"));
	}

	//Ensure that the rgb data taken from img after opening an image is
	//not empty and actually has some data in it.
	#[test]
	pub fn test_bin_data(){
		let mut img = Img::new(32, 32, 1);
		img.open_image("src/test_pic.jpg");

		assert_ne!(0, img.rgb_data.len());
	}

}