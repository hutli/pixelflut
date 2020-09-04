use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
//use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

const MAX_UNKNOWNS:u8 = 5;
const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 300;


#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    alive: bool,
    // Used for the trail effect. Always 255 if `self.alive` is true (We could
    // use an enum for Cell, but it makes several functions slightly more
    // complex, and doesn't actually make anything any simpler here, or save any
    // memory, so we don't)
    heat: u8,
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    let mut unknowns:u8 = 0;
    while match stream.read(&mut data) {
        Ok(size) => {
            let input = str::from_utf8(&data[0..size]).unwrap();
            match input.get(..3) {
                Some("PX ") => println!("PIXEL COMMAND"),
                _ => {
                    unknowns += 1;
                    if unknowns > MAX_UNKNOWNS{
                        stream.shutdown(Shutdown::Both).unwrap();
                    }
                },
            };
            print!("Input: {}", input);

            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        let size = window.current_monitor().size();
        (
            size.width as f64 / hidpi_factor,
            size.height as f64 / hidpi_factor,
        )
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round();

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}

fn new_empty(width: usize, height: usize) -> Self {
    assert!(width != 0 && height != 0);
    let size = width.checked_mul(height).expect("too big");
    Self {
        cells: vec![Cell::default(); size],
        scratch_cells: vec![Cell::default(); size],
        width,
        height,
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) = create_window("PixelFlut!", &event_loop);

    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    let mut life = new_empty(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture);

    let listener = TcpListener::bind("0.0.0.0:1337").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 1337");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}