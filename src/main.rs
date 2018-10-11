/*
	Video Resolution:
		Low:  QVGA  320x240
		Med:  VGA   640x480
		high: SXGA  1289x1024
*/
#[macro_use]
extern crate glium;
extern crate freenectrs;
extern crate image;
//use self::freenectrs::freenect::*;

use std::env;
use std::io;
use std::sync::{Arc, Mutex}; 
use std::thread;
use std::time::Duration;
use img::Img;
use std::sync::mpsc;
use input_device_handler::{ValidInp, DeviceHandler};

#[cfg(feature="window")]
use window::Window;
//use glium::Display;

mod flasch;
mod img;
mod input_device_handler;

#[cfg(feature="window")]
mod window;


const KINECT_NUM: u32 = 0;
const ARG_NUM: u32 = 4; // host, w, h, z


//use img::img;
fn main() {
	//colect the command line variables
	let args: Vec<String> = env::args().collect();
	
	//Check that we have valid args
	let info = confirm(&args[..]).unwrap();

	//set the host name
	let host: &str = &*info.0;

	//set up freenect with motor+video
	let context = freenectrs::freenect::FreenectContext::init_with_video_motor().expect("Could not set up Kinect w/ motor context");

	//Open kinect with given number
	let device = match context.open_device(KINECT_NUM) {
		Ok(x)	=>	x,
		Err(e)	=>	panic!("Error: {:?}", e),
	};


	//Set the kinect mode
	
	#[cfg(feature = "depth")]
	device.set_depth_mode(freenectrs::freenect::FreenectResolution::Medium, freenectrs::freenect::FreenectDepthFormat::MM).expect("Could not set depth mode!");
	#[cfg(feature = "depth")]
	let d_stream = device.depth_stream();

	#[cfg(feature = "rgb")]
	device.set_video_mode(freenectrs::freenect::FreenectResolution::Medium, freenectrs::freenect::FreenectVideoFormat::Rgb).expect("could not set Rgb mode");
	#[cfg(feature = "rgb")]
	let v_stream = device.video_stream();


	//get the depth stream
	#[cfg(feature = "depth")]
	let depth_stream = match d_stream {
		Ok(x)	=> x,
		Err(e)	=> panic!("Error: {:?}", e),
	};

	#[cfg(feature = "rgb")]
	let video_stream = match v_stream {
		Ok(x)	=> x,
		Err(e)	=> panic!("{:?}", e),
	};

	//start the main thread to process libfreenect events
	match context.spawn_process_thread() {
		Ok(_)	=> println!("Context thread spawned"),
		Err(e)	=> panic!("Error: {:?}", e),
	};

	//Set up the flaschen taschen display
	let fl = flasch::Flaschen::new(host, info.1, info.2);

	
	let  end = Arc::new(Mutex::new(false));
	let thread_end = end.clone();
	let (tx, rx) = mpsc::channel();

	let mut dev_angle = DeviceHandler{
							dev: &device,
							angle: 0 as f64,
							inp: ValidInp::Invalid,
						};

	dev_angle.reset(); //Reset the camera to start at 0

	#[cfg(feature = "window")]
	let mut window = Window::new(640, 480);
	//Set up the Window for viewing in glium
	

	//create a new thread that will waits for the user input. Thread is put to sleep if there is no input to be 
	//read so that it doesnt waste as many clock cycles when there is nothing. Once there is a match to quite we 
	//break the inf loop and let the thread exit.
	thread::spawn(move || {
		loop{
			let mut input = String::new();
			let found_end = match io::stdin().read_line(&mut input) {
				Ok(len)	=>	{if len > 0 && input.trim() == 'q'.to_string() { 
				 			    true
				 			 } 
				 			 else if input.trim() == "w".to_string() {
				 			 	println!("Moving angle up");
				 			 	tx.send(ValidInp::Up).unwrap();
				 			 	false
				 			 }
				 			 else if input.trim() == "s".to_string() {
				 			 	println!("moving angle down");
				 			 	tx.send(ValidInp::Down).unwrap();
				 			 	false
				 			 }
				 			 else {
				 			 	false
				 			 } 
				 			},
				Err(e)	=> panic!("{:?}", e),
			};
			
			if found_end == true{
				let mut end = thread_end.lock().unwrap();
				*end = true;

				return;
			}
				thread::sleep(Duration::from_millis(2));
		}
	});

	//Loop that takes the depth data from the kinect, creates a usuable data for the flaschen T. 
	//and sends the data to the display.
	//create a new image
	let i = Img::new(info.1, info.2, info.3);

		while !*end.lock().unwrap() {
			let mut i = i.clone();

			match rx.try_recv() {
				Ok(e)	=> dev_angle.key_event(e),
				Err(_)	=> (),
			}

			//if info.4 == "depth" {
			#[cfg(feature = "depth")]
			{
				if let Ok((data,_)) = depth_stream.receiver.try_recv() {
					i.convert_data_img(data);
					
					#[cfg(feature = "window")]
					{
						let pic = i.clone().get_pic();
						match pic {
							Some(p) => window.draw(p),
							None	=> (),
						} 
					}
					//Take data from kinect
				fl.send(i.binary_img());//send data to flaschen taschen display
				i.clear_data();
				}
			}
			//} 
			//else {
			#[cfg(feature = "rgb")]
			{
				if let Ok((data, _)) = video_stream.receiver.try_recv() {
					i.get_img(data);
					
					
					#[cfg(feature = "window")]
					{
						let pic = i.clone().get_pic();
						match pic {
							Some(p)	=> window.draw(p),
							None	=> (),
						};
					}
				}
				fl.send(i.binary_img());
				i.clear_data();	
			}
			
		}	
	#[cfg(any(feature = "rgb", feature = "depth"))]
	{
		dev_angle.reset();
		context.stop_process_thread().expect("Kinect process thread could not be closed");
	}

}

// ADDITIONAL HELPER FUNCTIONS 

//confirm command line arguments and return tuple(w,h)
 pub fn confirm(args: &[String]) -> Option<(String, u64, u64, u64)> {
 	//check for min num or args
 	if args.len()-1 != ARG_NUM as usize {
 		panic!("Incorect number of Args, need: {:?} args", ARG_NUM);
 	}
 	let host: String = args[1].parse::<String>().expect("not valid host name");
 	let h: u64 = args[2].parse::<u64>().expect("Not valid number");
 	let w: u64 = args[3].parse::<u64>().expect("Not valid number");
 	let z: u64 = args[4].parse::<u64>().expect("Not valid number");
 	//let mut stream: String = args[5].parse::<String>().expect("Not valid stream");

 	//println!("stream: {:?}", stream);
 	//Check to ensure that there is a valid video type, otherwise default into depth mode
 	//if stream.trim().to_lowercase() != "depth".trim().to_string() && stream.to_lowercase() != "rgb".to_string() {
 	//	stream = "depth".to_string();
 	//}

 	Some((host, h, w, z))

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
