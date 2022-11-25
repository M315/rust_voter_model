use rand::distributions::{Uniform, Bernoulli};
use rand::prelude::*;
use rand::seq::SliceRandom;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Duration;

fn render_state(canvas: &mut Canvas<Window>, board: &Vec<Vec<bool>>, width: usize, height: usize, w_rec: u32, h_rec: u32) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for i in 0..width {
        for j in 0..height {
            if board[i][j] {
                canvas.fill_rect(Rect::new((i * w_rec as usize) as i32, (j * h_rec as usize) as i32, w_rec, h_rec))?;
            }
        }
    }
    canvas.present();

    Ok(())
}

fn render_rec(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Color, w_rec: u32, h_rec: u32) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new((x * w_rec as i32) as i32, (y * h_rec as i32) as i32, w_rec, h_rec))?;
    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    // Init sdl
    let sdl_context = sdl2::init().expect("Unable to initialize SDL");
    let video_subsystem = sdl_context
        .video()
        .expect("Unable to initialize the video system");

    // Set initial parameters
    let width : u32 = 100;
    let height : u32 = 100;

    // Set point size
    let w_rec : u32 = 4;
    let h_rec : u32 = 4;

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
    let ber = Bernoulli::new(0.5).expect("Couldn't generate bernoulli random variable");
    let uni = Uniform::from(0..height as usize);
    let mut rng = rand::thread_rng();

    // Initialize state at time 0
    let mut t : u64 = 0;
    let mut state : Vec<Vec<bool>> = Vec::new();
    
    for _ in 0..width as usize {
        let col : Vec<bool> = rand::thread_rng().sample_iter(ber).take(height as usize).collect();
        state.push(col);
    }

    render_state(&mut canvas, &state, width as usize, height as usize, w_rec, h_rec)?;

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

        let site : Vec<usize> = uni.sample_iter(&mut rng).take(2).collect();
        let dir : (i32, i32) = *[(1, 0), (0, 1), (-1, 0), (0, -1)].choose(&mut rng).expect("Couldn't choose");
        let neighbor : (usize, usize) = (
            (width as i32 + site[0] as i32 + dir.0) as usize % width as usize,
            (height as i32 + site[1] as i32 + dir.1) as usize % height as usize
        );
        state[site[0]][site[1]] = state[neighbor.0][neighbor.1];

        let mut color : Color = Color::RGB(255, 255, 255);
        if state[neighbor.0][neighbor.1] {
            color = Color::RGB(0, 0, 0);
        }

        if t % 1_000 == 0 {
            render_rec(&mut canvas, site[0] as i32, site[1] as i32, color, w_rec, h_rec)?;
        }
    }

    println!("{}", t);

    Ok(())
}

