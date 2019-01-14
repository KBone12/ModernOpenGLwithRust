use gl;

use glutin::{
    Api,
    ContextBuilder,
    Event,
    EventsLoop,
    GlContext,
    GlProfile,
    GlRequest,
    GlWindow,
    WindowBuilder,
    WindowEvent,
    dpi::LogicalSize,
};
#[cfg(target_os = "linux")] use glutin::os::unix::MonitorIdExt;
#[cfg(target_os = "macos")] use glutin::os::macos::MonitorIdExt;
#[cfg(target_os = "windows")] use glutin::os::windows::MonitorIdExt;

fn show_monitors_info(events: &EventsLoop) {
    println!("========================");
    println!("   available monitors   ");
    println!("========================");
    events.get_available_monitors().for_each(|monitor| {
        println!("monitor {}:", monitor.native_id());
        if monitor.native_id() == events.get_primary_monitor().native_id() {
            println!("\tprimary ");
        }
        if let Some(name) = monitor.get_name() {
            println!("\tname: {}", name);
        }
        println!("\tposition: ({}, {})", monitor.get_position().x, monitor.get_position().y);
        println!("\tsize: {} x {}", monitor.get_dimensions().width, monitor.get_dimensions().height);
        println!("\tdots per inch: {}\n", monitor.get_hidpi_factor());
    });
}

fn main() {
    let mut events = EventsLoop::new();
    let window_builder = WindowBuilder::new()
                            .with_title("Modern OpenGL with Rust")
                            .with_dimensions(LogicalSize::new(400.0, 300.0))
                            .with_resizable(true);
    let context_builder = ContextBuilder::new()
                            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
                            .with_gl_profile(GlProfile::Core)
                            .with_vsync(true)
                            .with_double_buffer(Some(true));
    let window = GlWindow::new(window_builder, context_builder, &events).unwrap();

    show_monitors_info(&events);
    unsafe {
        window.make_current().unwrap();
        gl::load_with(|address| window.get_proc_address(address) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut running = true;
    while running {
        events.poll_events(|event| {
            match event {
                Event::WindowEvent { event, ..} => {
                    match event {
                        WindowEvent::CloseRequested => running = false,
                        _ => {}
                    }
                },
                _ => {}
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers().unwrap();
    }
}
