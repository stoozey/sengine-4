use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{event, EventPump};
use sdl2::event::Event;

fn main() -> Result<(), String> {
    let context = sdl2::init()?;
    let video_subsystem = context.video()?;
    let window = video_subsystem.window("Penis!", 1280, 720)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let mut running = true;
    let mut event_queue = context.event_pump().unwrap();

    let mut x = 0;
    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                }

                _=> { }
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let rect = Rect::new(x, 0, 64, 64);
        x += 1;

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(rect)?;
        canvas.present();
    }

    Ok(())
}