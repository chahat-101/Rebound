use winit::{

    application::ApplicationHandler, 
    event::{self, Event, WindowEvent}, 
    event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop}, 
    window::{self, Window, WindowId}
};

use pixels::{Pixels,SurfaceTexture};



fn main(){
    let event_loop = EventLoop::new().unwrap();

   let window = event_loop.create_window(Window::default_attributes()).expect("failed to load window");

    let size = window.inner_size();

    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);

    let mut pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();


    event_loop.run(|event,active_event_loop|{

        match event{
            Event::WindowEvent {event:WindowEvent::CloseRequested,
                ..
            } =>{
                active_event_loop.exit();
            },

            Event::WindowEvent { 
                event: WindowEvent::Resized(size),
                ..
            } => {
                pixels.resize_surface(size.width, size.height);
                pixels.resize_buffer(size.width, size.height);
                window.request_redraw();
            },

            Event::WindowEvent { 
                event:WindowEvent::RedrawRequested,
                ..
            } => {

                let frame = pixels.frame_mut();

                for px in frame.chunks_exact_mut(4){
                    px.copy_from_slice(&[255,255,255,255]);
                }

                pixels.render().unwrap();
            },

            _=> {},

        }

       

    });


}