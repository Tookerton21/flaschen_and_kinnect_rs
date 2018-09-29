/*
	Video Resolution:
		Low:  QVGA  320x240
		Med:  VGA   640x480
		high: SXGA  1289x1024
*/

extern crate freenectrs;

//use self::freenectrs::freenect::*;

use std::env;

mod flasch;
mod img;


use img::Img;

const KINECT_NUM: u32 = 0;

//use img::img;
fn main() {
	//set up freenect with motor+video
	let context = freenectrs::freenect::FreenectContext::init_with_video().expect("Could not set up Kinect w/ motor context");

	//Open kinect with given number
	let device = match context.open_device(KINECT_NUM) {
		Ok(x)	=>	x,
		Err(e)	=>	panic!("Error: {:?}", e),
	};

	//Set the kinect mode
	device.set_depth_mode(freenectrs::freenect::FreenectResolution::Medium, freenectrs::freenect::FreenectDepthFormat::MM).expect("Could not set depth mode!");

	//get the depth stream
	let depth_stream = match device.depth_stream() {
		Ok(x)	=> x,
		Err(e)	=> panic!("Error: {:?}", e),
	};

	//check what the current camera angle is at
	let cur_angle = device.get_tilt_degree().expect("couldnt get camera angle.");
	println!("Current angle: {:?}", cur_angle);

	//depth camera to reset to 0 degrees
	device.set_tilt_degree(2.0).expect("Error with setting camera angle");
	

	//start the main thread to process libfreenect events
	match context.spawn_process_thread() {
		Ok(_)	=> println!("Main thread spawned"),
		Err(e)	=> panic!("Error: {:?}", e),
	};


	loop{
		//Fetch the depth frames
		if let Ok((data,_)) = depth_stream.receiver.try_recv() {
			println!("Inside fetch data frames");
		}
	}		
	context.stop_process_thread();

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
