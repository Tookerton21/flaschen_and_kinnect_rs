//Handles the user input when it comes to moving the camera. If the user
//chooses to move either up or down, and adjust the camera based from that
//on the kinect.

use freenectrs::freenect;


const UP: f64 = 5.0;
const DOWN:f64 = -5.0;

pub enum ValidInp {
	Up,
	Down,
	Invalid,
}

impl PartialEq for ValidInp {
	fn eq(&self, other: &ValidInp) -> bool {
		if self == other {
			true
		}
		else{
			false
		}
	}
}


pub struct DeviceHandler <'a, 'b: 'a> {
	pub dev: &'a freenect::FreenectDevice<'a, 'b>,
	pub angle: f64,
	pub inp: ValidInp,

}

impl <'a, 'b> DeviceHandler <'a, 'b> {
	pub fn new(dev: &'a freenect::FreenectDevice<'a, 'b>) -> DeviceHandler<'a, 'b> {
		DeviceHandler {
			dev: &dev,
			angle: 0.0,
			inp: ValidInp::Invalid,
		}
	}
	//Reset the angle of the kinect back to 0.0
	pub fn reset(&mut self) {
		self.angle = 0.0;
		self.dev.set_tilt_degree(self.angle).expect("Could not reset the angle");
	}

	//Check the input from the user, and see if it is a valid key event
	pub fn key_event(&mut self, input: ValidInp){
		match input {
			ValidInp::Up   => self.move_angle(UP),
			ValidInp::Down => self.move_angle(DOWN),
			_	   => self.inp = ValidInp::Invalid,
		}
	}

	//Move the camera based on the users key input
	fn move_angle(&mut self, add: f64) {

		self.dev.set_tilt_degree(self.angle + add).expect("Error with setting camera angle");
		self.angle = self.dev.get_tilt_degree().expect("could not get degree");
		println!("tilt_angle from dev: {:?}", self.dev.get_tilt_degree());
	}	

}