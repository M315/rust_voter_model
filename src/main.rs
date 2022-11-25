use rand::distributions::{Uniform, Bernoulli};
use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

// TODO: Improve render
fn render_board(canvas: &mut Canvas<Window>, board: &Vec<Vec<bool>>, first_col : usize, width: usize, height: usize, w_rec: u32, h_rec: u32) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for i in 0..width {
        for j in 0..height {
            if board[i + first_col][j] {
                canvas.fill_rect(Rect::new((i * w_rec as usize) as i32, (j * h_rec as usize) as i32, w_rec, h_rec))?;
            }
        }
    }
    canvas.present();

    ::std::thread::sleep(Duration::new(0u64,  1_000_000u32));
    Ok(())
}

fn main() -> Result<(), String> {
    // Init sdl
    let sdl_context = sdl2::init().expect("Unable to initialize SDL");
    let video_subsystem = sdl_context
        .video()
        .expect("Unable to initialize the video system");

    // Set initial parameters
    let width : u32 = 800;
    let height : u32 = 800;

    // Set point size
    let w_rec : u32 = 1;
    let h_rec : u32 = 1;

    // Generate a window
    let window = video_subsystem
        .window("rust-sdl2 demo", width * w_rec, height * h_rec)
        .position_centered()
        .build()
        .unwrap();

    // Init the canvas and paint it black
    let mut canvas : Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Could not make a canvas");
    // Set color
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // Paint the hole canvas
    canvas.clear();
    canvas.present();

    // Initialize the event tracker
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialize the random genreators
    let uni = Uniform::from(0..width);
    let ber = Bernoulli::new(0.5).expect("Couldn't generate bernoulli random variable");

    // Initialize state at time 0
    let mut t : u64 = 0;
    let mut state : Vec<bool> = rand::thread_rng().sample_iter(ber).take(height as usize).collect();

    // Initialize empty board
    let mut board : Vec<Vec<bool>> = vec![vec![false; height as usize]; width as usize];
    let mut shift : u64 = 0;

    // Set last column to state 0
    //board[width as usize - 1] = state;
    board[0] = state;

    // Video/Game loop
    'running: loop {
        // Check if a breaking event has been recorded
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Generate next state and update board
        t += 1;
        state = rand::thread_rng().sample_iter(&ber).take(height as usize).collect();
        board.push(state);

        render_board(&mut canvas, &board, (t - shift) as usize, width as usize, height as usize, w_rec, h_rec)?;

        // TODO: Trim board
        if board.len() > 2 * width as usize {
            //shift += width as u64;
            break 'running;
        }
        println!("{}", t);
    }
    Ok(())
}

