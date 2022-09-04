mod transform;
mod camera;
mod samumath;
mod object;

use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::BufReader;
use glium::uniform;
use obj::{load_obj, Obj, Position};
use camera::Camera;
use transform::Transform;


fn main() {
    use glium::{glutin, Surface};


    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let exe_path = std::env::current_exe().unwrap();
    let build_path = exe_path.parent().expect("Executable must be in some directory");
    let input = BufReader::new(File::open(Path::join(build_path, "resources/objs/teapot.obj")).unwrap());
    let obj: Obj<Position> = load_obj(input).unwrap();

    let vertex_buffer = obj.vertex_buffer(&display).unwrap();
    let index_buffer = obj.index_buffer(&display).unwrap();

    let vertex_shader_path = Path::join(build_path, "resources/shaders/simple.vert");
    let fragment_shader_path = Path::join(build_path, "resources/shaders/simple.frag");
    let vertex_shader_src = fs::read_to_string(vertex_shader_path).expect("Error reading frag shader data");
    let fragment_shader_src = fs::read_to_string(fragment_shader_path).expect("Error reading vert shader data");
    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();


    let mut transform = transform::Transform::new(glam::Vec3::new(1.0, 1.0, 1.0), glam::Quat::IDENTITY, glam::Vec3::ZERO);
    transform.rotate_y(45.0);
    let transform_matrix = transform.to_mat4();


    let mut camera = Camera::new(Transform::EMPTY, 0.1, 10000.0, 96.0, 1.0);
    camera.transform.rotate_y(30.0);
    camera.transform.translate(glam::Vec3::new(0.0, 0.0, 10.0));
    let view_matrix = camera.to_view_matrix();

    let model_view =  view_matrix * transform_matrix;
    let perspective = camera.to_perspective_matrix();
    let mvp = perspective * model_view;
    
    println!("Camera transform: {}", camera.transform.to_mat4());
    println!("View Matrix: {}", view_matrix);
    println!("World Matrix: {}", transform_matrix);
    println!("ModelView Matrix: {}", model_view);


    let uniforms = uniform! {

        model: mvp.to_cols_array_2d()
    };
    
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };        
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms,
                    &params).unwrap();
        target.finish().unwrap();
    });
}