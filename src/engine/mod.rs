use sdl2::video::Window;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;

pub struct Engine {
    context: Sdl,
    window: Window,
    canvas: WindowCanvas,
    loop_running: bool,
}

impl Engine {
    pub fn create() -> Result<Engine, String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;
        let window = video_subsystem.window("Penis!", 1280, 720)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .build()
            .unwrap();

        Ok(Engine { context, window, canvas, loop_running: false })
    }

    pub fn run_loop(&mut self) {
        loop_running = true;

        let mut event_queue = context.event_pump().unwrap();
        while loopRunning {
            for event in event_queue.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        loop_running = false;
                    },

                    _=> { }
                }
            }

            // TODO add update loop

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();

            // TODO add render loop

            canvas.present();
        }
    }
}