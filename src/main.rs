mod transform;
mod camera;
mod samumath;
mod object;
mod light;

use std::fs;
use std::path::Path;
use glium::uniform;
use camera::Camera;
use transform::Transform;
use glam::{Mat3, Vec3};
use glam::Vec4Swizzles;
use light::Light;
use std::time::SystemTime;
fn main() {
    use glium::{glutin, Surface};


    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let exe_path = std::env::current_exe().unwrap();
    let build_path = exe_path.parent().expect("Executable must be in some directory");
    // let input = BufReader::new(File::open(Path::join(build_path, "resources/objs/teapot.obj")).unwrap());
    // let obj: Obj<Position> = load_obj(input).unwrap();

    // let vertex_buffer = obj.vertex_buffer(&display).unwrap();
    // let index_buffer = obj.index_buffer(&display).unwrap();



    let vertex_shader_path = Path::join(build_path, "resources/shaders/phong.vert");
    let fragment_shader_path = Path::join(build_path, "resources/shaders/phong.frag");
    let vertex_shader_src = fs::read_to_string(vertex_shader_path).expect("Error reading frag shader data");
    let fragment_shader_src = fs::read_to_string(fragment_shader_path).expect("Error reading vert shader data");
    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    // Create Teapot
    let mut transform = transform::Transform::new(glam::Vec3::new(1.0, 1.0, 1.0), glam::Quat::IDENTITY, Vec3::new(-0.25, -1.0, 0.0));
    transform.rotate_y(45.0);
    let mut teapot = object::Object::from_file(Path::join(build_path, "resources/objs/teapot.obj"), transform).unwrap();
    let vertex_buffer = teapot.get_vertex_buffer(&display).unwrap();
    let index_buffer = teapot.get_index_buffer(&display).unwrap();

    // Create camera
    let mut camera = Camera::new(Transform::EMPTY, 0.1, 10000.0, 96.0, 1.0);
    camera.transform.translate(Vec3::new(0.0, 5.0, 5.0));
    camera.transform.rotate_x(-45.0);
    let view_matrix = camera.to_view_matrix();

    // Create Light
    let mut light = Light::WHITE;
    light.transform.translate(Vec3::new(10.0, 30.0, 0.0));

    // let model_view =  view_matrix * transform_matrix;
    // let perspective = camera.to_perspective_matrix();
    // let mvp = perspective * model_view;
    
    // println!("Camera transform: {}", camera.transform.to_mat4());
    // println!("View Matrix: {}", view_matrix);
    // println!("World Matrix: {}", transform_matrix);
    // println!("ModelView Matrix: {}", model_view);

    let startTime = SystemTime::now();
    event_loop.run(move |event, _, control_flow| {

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => {



                    
                },
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let seconds = (SystemTime::now().duration_since(startTime).unwrap().as_secs_f32() * 4.0).cos() * 50.0;
        // println!("{}", seconds);
        // println!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f64());
        let timeoffset = Vec3::new(seconds, 0.0, 0.0);
        light.transform.translate(timeoffset);
        
        teapot.transform.rotate_y(1.0);
        let model_view =  view_matrix * teapot.transform.to_mat4();
        let perspective = camera.to_perspective_matrix();
        let view_matrix_mat3 = Mat3::from_cols(view_matrix.x_axis.xyz(), view_matrix.y_axis.xyz(), view_matrix.z_axis.xyz());
        let light_pos_view = view_matrix_mat3 * light.transform.translation;
        
        //println!("{}", light_pos_view);

        let normal_matrix = Mat3::from_cols(model_view.x_axis.xyz(), model_view.y_axis.xyz(), model_view.z_axis.xyz()).transpose().inverse();
        let uniforms = uniform! {
            model_view: model_view.to_cols_array_2d(),
            perspective: perspective.to_cols_array_2d(),
            normal_matrix: normal_matrix.to_cols_array_2d(),
            light_position: light_pos_view.to_array()
        };
        
        light.transform.translate(-timeoffset);

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