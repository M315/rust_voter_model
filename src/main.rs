use rand::distributions::{Distribution, Uniform};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

fn main() -> Result<(), String> {
    // Init sdl
    let sdl_context = sdl2::init().expect("Unable to initialize SDL");
    let video_subsystem = sdl_context
        .video()
        .expect("Unable to initialize the video system");

    // Generate a window
    let window = video_subsystem
        .window("rust-sdl2 demo", 50 * 20, 50 * 20)
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
    //canvas.clear();
    //canvas.present();

    // Initialize the event tracker
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialize the random genreators
    let uni = Uniform::from(0..4);
    let mut rng = rand::thread_rng();

    // Initial possition
    let mut x = 25;
    let mut y = 25;

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
                x += 1;
                y += 1
            }

            1 => {
                x -= 1;
                y += 1
            }

            2 => {
                x -= 1;
                y -= 1
            }

            3 => {
                x += 1;
                y -= 1
            }
            _ => {}
        }

        if x > 50 || x < 0 || y > 50 || y < 0 {
            x = 25;
            y = 25;
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_point(Point::new(x, y))?;
        canvas.set_scale(20., 20.).expect("Coudl not rescalate");

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1000u32));
    }
    Ok(())
}
