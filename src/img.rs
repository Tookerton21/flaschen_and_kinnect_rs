//The img struct is to be used to pass the data image in a udp packet. The header is, and 
//following rgb data vector are used for a raw ppm file. 
extern crate image;

#[derive(Debug, Clone)]
pub struct img {
	header:  String,
	rgb_data: Vec<u8>,
}

impl img{
	//Creates a new PPM file, with the required header for the flaschen taschen project, 
	//and an empty vector to hold the binary information from the rgb data of the image.
	pub fn new(height: u64, width: u64, z_layer: u64) -> img{
		let h = format!("P6\n{h} {w}\n#FT: {h} {w} {z}\n255", h = height, w=width, z=z_layer);
		img {
			header : h,
			rgb_data: Vec::new(),
		}
	}

	//Taken the address of the image and open it using the image crate. Get the raw
	//Rgb data from the Dynamicimage. Save this Vec<u8> into the image structure
	pub fn open_image(& mut self, img_file: &str) ->  bool {
		let img = match image::open(img_file) {
			Ok(t) =>	t,
			Err(_)	=>	panic!("ERROR In opening the file"),
		};
		self.rgb_data = img.to_rgb().into_raw();
		true
	}

	//Return the raw PPM data into a since binary vec to be sent as a udp packet
	pub fn binary_img(&self) -> Vec<u8> {
		let mut h = self.header.clone().into_bytes();
		h.append(&mut self.rgb_data.clone());
		h
	}
}