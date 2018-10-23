//The img struct is to create a raw PPM file that can be used to take any

extern crate image;

use self::image::{GenericImage};

#[cfg(feature = "rgb")]
use self::image::DynamicImage;

//Default setting for Display width and size
const SIZE: (u64, u64) = (25,35);
const DISPLAY: (u32, u32) = (640, 480); //(wxh)

#[cfg(feature = "depth")]
const MIN: u16 = 0; //CHANGE THIS IF YOU WANT TO SET A MIN DISTANCE YOU WANT TO CAPTURE ON DISPLAY FROM DEPTH SENSOR

#[derive(Clone)]
pub struct Img {
	height: u64,
	width: u64,
	header:  String,
	rgb_data: Vec<u8>,
	picture: Option<image::DynamicImage>,
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
			picture: None,
		}
	}

	//Take the a depth point and assign it a color according to its distance from the camera.
	//Return this point with the color assigned. For the kinect v1, the raw depth values range
	//between 0 and 2048 
	#[cfg(feature = "depth")]
	pub fn get_color(dist: u16) -> image::Rgba<u8> {
		
		match dist {
			MIN...410	=>	image::Rgba([255, 0, 0, 255]),    //Red color
			MIN...820  =>	image::Rgba([255, 255, 0,255 ]),  //Yellow color
			MIN...1230  =>  image::Rgba([0, 255, 0, 255]),	  //Green Color
			MIN...1640  =>  image::Rgba([0, 0, 255, 0]), //Blue Color
			_			=>  image::Rgba([0,0,0, 255]),   //Black Color
		}

	} 
	//Takes a raw depth data array and turn the distance point into abn rgb dynamic image
	//, resizes it and then copys the rgb data to the image struct. 
	#[cfg(feature = "depth")]
	pub fn convert_data_img(&mut self, data: &[u16]){
		let mut img: image::DynamicImage = image::DynamicImage::new_rgb8(DISPLAY.0, DISPLAY.1);
		let mut pos = 0;
		
		for y in 0..DISPLAY.1 {
			for x in 0..DISPLAY.0 {
				if pos > data.len(){
					panic!("Outside of bounds!!");
				}
				
				img.put_pixel(x,y, self::Img::get_color(data[pos]));
				pos += 1;
			}
		}

		//create an rgb image of the picture from the kinnect
		self.picture = Some(img.clone());

		//resize the image to fit on the flaschen display
		self.rgb_data = img.resize(self.width as u32, self.height as u32, image::FilterType::Nearest).to_rgb().into_raw();
		
	}

	//returns the image from the kinect without it being scaled and should be coming in 
	//at 640x480
	#[cfg(feature = "window")]
	pub fn get_pic(self) -> Option <image::RgbaImage> {
		match self.picture	{
			Some(result)	=>	Some(result.to_rgba()),
			None			=>	None,
		}
	}

	#[cfg(feature = "rgb")]
	pub fn get_img(&mut self, data: &[u8]) {
		let mut img: DynamicImage = image::DynamicImage::new_rgb8(DISPLAY.0, DISPLAY.1);
		let mut pos = 0;
		let inc = 3;

		for y in 0..DISPLAY.1 {
			for x in 0..DISPLAY.0 {
				if pos > data.len() {
					panic!("Outside of bounds!!");
				}
				img.put_pixel(x,y, image::Rgba([data[pos], data[pos+1], data[pos+2], 255]));
				pos = pos+inc;
			}
		}

		self.picture = Some(img.clone());
		self.rgb_data = img.resize(self.width as u32, self.height as u32, image::FilterType::Nearest).to_rgb().into_raw();

	}
	//Taken the address of the image and open it using the image crate. Get the raw
	//Rgb data from the Dynamicimage after resizing it to fit the flaschen display.
	//Save this Vec<u8> into the image structure. Returns true if the image was able 
	//to open and false is unable to open the image.
	#[test]
	pub fn open_image(& mut self, img_file: &str) ->  bool {
		use std::path::Path;
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