//#[macro_use]
//extern crate glium;

use glium;
use glium::glutin;
use glium::index::PrimitiveType;
use glium::texture::{RawImage2d, Texture2d};
use image::RgbaImage; 
use glium::Surface;


#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub struct Window {
	//window_info: WindowInfo,
	buff: BufferInfo,
	shader: Shader,
	display: glium::Display,
	//context: glutin::ContextBuilder<'a>,
}

impl  Window {
	pub fn new(w: f64, h: f64) -> Window {
		let win_info = WindowInfo::new(w,h);

		let context = glutin::ContextBuilder::new();

		//let win_info = WindowInfo::new(w,h);
		let d = glium::Display::new(win_info.window.clone(), context, &win_info.events_loop).unwrap();
		let b = BufferInfo::new(&d);

		Window {
			//window_info: win_info,
			buff: b,
			shader: Shader::new(&d),
			display: d,
		}
	}

	//Take the image 
	pub fn image(&mut self, img: RgbaImage) -> Texture2d { 
		//let dim = img.dimension();
		let image = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), (640, 480));

		//Get the texture of the timage
		Texture2d::new(&self.display, image).expect("Could not texturize")
	}

	pub fn draw(&mut self, img: RgbaImage){
		let mut target = self.display.draw();
		target.clear_color(0.0,0.0,1.0,1.0);

		let texture = self.image(img);

		let uniforms = uniform! {
           matrix: [
               [1.0, 0.0, 0.0, 0.0],
               [0.0, 1.0, 0.0, 0.0],
               [0.0, 0.0, 1.0, 0.0],
               [0.0, 0.0, 0.0, 1.0f32],
            ],
           	tex: &texture,
        };

        let (vertex, index) = self.buff.get_buffs();
        target.draw(vertex, index, self.shader.get_prog(), &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
		
	}
}


pub struct Shader {
	prog: glium::Program,

}

impl Shader {
	pub fn new(display: &glium::Display) -> Shader{
		let prog = glium::Program::from_source(display, r" 
		#version 140

		uniform mat4 matrix;

		in vec2 position;
		in vec2 tex_coords;
		out vec2 v_tex_coords;


		void main() {
   		v_tex_coords = tex_coords;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
		}

	", r"
	#version 140
	
	uniform sampler2D tex;
    in vec2 v_tex_coords;
    out vec4 color;


    void main() {
    	color = texture(tex, v_tex_coords);
    }
	", None).unwrap();
		Shader {
			prog: prog,
		}
	}
	
	pub fn get_prog(&self) -> &glium::Program {
		&self.prog
	}
}

pub struct BufferInfo {
	vertex_buffer: glium::VertexBuffer<Vertex>,
	pub index_buffer: glium::IndexBuffer<u16>,
}

#[warn(private_in_public)]
impl BufferInfo {
	pub fn get_buffs(&self) -> (&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u16>){
		(&self.vertex_buffer, &self.index_buffer)
	}

	pub fn new(display: &glium::Display) -> BufferInfo{
		//use Vertex;
		//let data: [u16; 4] = [1, 2, 0, 3];
		let v = [Vertex{position: [-1.0, -1.0], tex_coords: [0.0, 0.0]},
			 	 Vertex{position: [-1.0, 1.0],  tex_coords: [0.0, 1.0]},
			 	 Vertex{position: [1.0, 1.0] ,  tex_coords: [1.0, 1.0]},
				 Vertex{position: [1.0, -1.0],  tex_coords: [1.0, 0.0]}
			 	];

		BufferInfo {
			vertex_buffer: glium::VertexBuffer::new(display, &v).expect("could not create vertex Buffer"),
			index_buffer: glium::IndexBuffer::new(display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0 ,3]).expect("could not create index buffer"),
		}
	}
}

pub struct WindowInfo {
	pub events_loop: glutin::EventsLoop,
	pub window: glutin::WindowBuilder, 
}

impl WindowInfo {
	pub fn new(width: f64, height: f64) -> WindowInfo  {
		
		 WindowInfo {
					events_loop: glutin::EventsLoop::new(),
					window: glutin::WindowBuilder::new()
					  			     .with_dimensions(glutin::dpi::LogicalSize::new(width, height))
					  				 .with_title("Kinect"),
		}
	}
}


