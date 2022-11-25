use rand::distributions::{Distribution, Uniform};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    // Init sdl
    let sdl_context = sdl2::init().expect("Unable to initialize SDL");
    let video_subsystem = sdl_context
        .video()
        .expect("Unable to initialize the video system");

    // Set initial parameters
    let width : u32 = 200;
    let height : u32 = 200;

    let w_rec : u32 = 4;
    let h_rec : u32 = 4;

    // Generate a window
    let window = video_subsystem
        .window("rust-sdl2 demo", width * w_rec, height * h_rec)
        .position_centered()
        .build()
        .unwrap();

    // Init the canvas and paint it black
    let mut canvas = window
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
    let uni = Uniform::from(0..4);
    let mut rng = rand::thread_rng();

    // Initial possition
    let mut x: i32 = (width / 2u32) as i32;
    let mut y: i32 = (height / 2u32) as i32;

    let mut i = 0;

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

        let dir = uni.sample(&mut rng);

        match dir {
            0 => {
                x += w_rec as i32;
                y += h_rec as i32
            }

            1 => {
                x -= w_rec as i32;
                y += h_rec as i32
            }

            2 => {
                x -= w_rec as i32;
                y -= h_rec as i32
            }

            3 => {
                x += w_rec as i32;
                y -= h_rec as i32
            }
            _ => {}
        }

        x = (x + (width * w_rec) as i32) % (width * w_rec) as i32;
        y = (y + (height * h_rec) as i32) % (height * h_rec) as i32;

        canvas.set_draw_color(Color::RGB(255 - i, i, 255 - i));
        canvas.fill_rect(Rect::new(x, y, w_rec, h_rec))?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 10_000u32));

        i = (i + 1) % 255;
    }
    Ok(())
}

