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
	//Reset the angle of the kinect back to 0.0
	pub fn reset(&mut self) {
		self.angle = 0.0;
		self.dev.set_tilt_degree(self.angle).expect("Could not reset the angle");
	}

	pub fn key_event(&mut self, input: ValidInp){
		match input {
			ValidInp::Up   => self.move_angle(UP),
			ValidInp::Down => self.move_angle(DOWN),
			_	   => self.inp = ValidInp::Invalid,
		}
	}

	fn move_angle(&mut self, add: f64) {
		//let tilt_angle: f64 = self.angle + add;
		//println!("Moving: {:?} degs", tilt_angle);

		self.dev.set_tilt_degree(self.angle + add).expect("Error with setting camera angle");
		self.angle = self.dev.get_tilt_degree().expect("could not get degree");
		println!("tilt_angle from dev: {:?}", self.dev.get_tilt_degree());
	}	

}