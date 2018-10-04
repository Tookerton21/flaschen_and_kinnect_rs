/*
	Video Resolution:
		Low:  QVGA  320x240
		Med:  VGA   640x480
		high: SXGA  1289x1024
*/

extern crate freenectrs;

//use self::freenectrs::freenect::*;

use std::env;
use std::io;
use std::thread;
use std::time::Duration;
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


	//For TESTING THE FLASCHEN DISPLAY

	//Set up the flaschen taschen display
	let fl = flasch::Flaschen::new("Localhost", 20, 20);

	//create a new image
	let mut i = img::Img::new(20, 20, 0);

	let mut end = false;
	let mut input = String::new();

	//create a new thread that will waits for the user input. Thread is put to sleep if there is no input to be 
	//read so that it doesnt waste as many clock cycles when there is nothing. Once there is a match to quite we 
	//break the inf loop and let the thread exit.
	thread::spawn(move || {
		loop {
			match io::stdin().read_line(&mut input) {
				Ok(len)	=>	{ if len > 0 && input == 'q'.to_string() {end = true; break}},
				Err(_)	=> { println!("Inside error for stdin() "); thread::sleep(Duration::from_millis(2))},
			}

			thread::sleep(Duration::from_millis(2));
		}
	});

	//Loop that takes the depth data from the kinect, creates a usuable data for the flaschen T. 
	//and sends the data to the display.
	while end != true {
		//Fetch the depth frames
		if let Ok((data,_)) = depth_stream.receiver.try_recv() {
			println!("Inside fetch data frames");
			i.convert_data_img(data); //Take data from kinect
			fl.send(i.binary_img());//send data to flaschen taschen display

		}
	}		

	context.stop_process_thread().expect("Kinect process thread could not be closed");

}

#[cfg(test)]
mod tests{
	use super::*;

	//Test to ensure that the that one can send an image to display on 
	//the flaschen taschen server, and server must be running during the 
	//test!!
	#[test]
	pub fn test_single_img(){
		//For testing out the flashen tashen dimensions
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
