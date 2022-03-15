
use std::time::{Instant, Duration};

use glow::{Context, HasContext};
use glutin::{event_loop::EventLoop, window::Window, ContextWrapper, PossiblyCurrent};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

use crate::world::{World};

pub struct WindowHandler {
    gl: Context,
    window: ContextWrapper<PossiblyCurrent, Window>,
    event_loop: EventLoop<()>,
}

impl WindowHandler {
    pub fn new(width: i32, height: i32, vsync: bool) -> Self {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(env!("CARGO_BIN_NAME"))
            .with_inner_size(glutin::dpi::LogicalSize::new(width as f32, height as f32));
        unsafe {
            let window = glutin::ContextBuilder::new()
                .with_vsync(vsync)
                .with_depth_buffer(24)
                .with_double_buffer(Some(true))
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
            let gl =
                glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            Self {
                gl,
                window,
                event_loop,
            }
        }
    }

    pub fn run(self) {
        let WindowHandler {
            gl,
            window,
            event_loop,
        } = self;

        let world = World::new(100, 200);


        {
            let mut updates = 0;
            let mut renders = 0;
            let update_timer = Instant::now();
            let second = 1_000_000_000;
            let tick_rate: u64 = 64;
            let tick_time: u64 = second as u64 / tick_rate;
            let mut tick_timer = 0;
            let mut cumulative_time: u128 = 0;

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;
                match event {
                    Event::LoopDestroyed => (),
                    Event::MainEventsCleared => {
                        while cumulative_time < update_timer.elapsed().as_nanos(){
                            cumulative_time += tick_time as u128;
                            updates += 1;
                            world.update();
                        }

                        if tick_timer < update_timer.elapsed().as_nanos(){
                            tick_timer += second;
                            println!("Updates: {} Renders: {}", updates, renders);
                            updates = 0;
                            renders = 0;
                        }

                        window.window().request_redraw();
                    }
                    Event::RedrawRequested(_) => {
                        renders += 1;
                        window.swap_buffers().unwrap();
                    }
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            window.resize(**new_inner_size);
                        }
                        WindowEvent::Resized(size) => unsafe {
                            gl.viewport(0, 0, size.width as i32, size.height as i32);
                        },
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => (),
                    },
                    _ => (),
                }
            });
        }
    }
}