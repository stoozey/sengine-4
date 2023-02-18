use sdl2::video::Window;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;

pub struct Engine {
    context: Sdl,
    canvas: WindowCanvas,
    loop_running: bool,
}

impl Engine {
    pub fn create(window_width: u32, window_height: u32) -> Result<Engine, String> {
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = video.window("Penis!", window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .build()
            .unwrap();

        Ok(Engine{ context, canvas, loop_running: false })
    }

    pub fn run_loop(&mut self) {
        self.loop_running = true;

        let mut event_queue = self.context.event_pump().unwrap();
        while self.loop_running {
            for event in event_queue.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        self.loop_running = false;
                    },

                    _=> { }
                }
            }

            // TODO add update loop

            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.clear();

            // TODO add render loop

            self.canvas.present();
        }
    }
}