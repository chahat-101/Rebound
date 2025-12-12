use winit::{
    event::{Event,WindowEvent},
    event_loop::{ControlFlow,EventLoop,ActiveEventLoop},
    window::{Window,WindowId},
    application::ApplicationHandler,
    
};
#[derive(Default)]
struct App{
    window:Option<Window>,

}


impl ApplicationHandler for App{

    fn resumed(&mut self, event_loop: & ActiveEventLoop) {
        
        let win = event_loop.create_window(Window::default_attributes())
                            .expect("failed to create window");
        
        self.window = Some(win);
        
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ) {
        
            match event {
            WindowEvent::CloseRequested => {
                // drop the window (it closes) and exit the event loop
                self.window = None;
                event_loop.exit();
            }
            _ => {}
        }
    }
}

fn main() {

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::default();


    event_loop.run_app(&mut app).unwrap();

}
