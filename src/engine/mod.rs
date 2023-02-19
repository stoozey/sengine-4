pub mod loop_runner;
pub mod entity;

use std::default;
use sdl2::video::Window;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode::O;

use entity::Entity;

static CLEAR_COLOUR: Color = Color::WHITE;

pub struct Engine<'a> {
    loop_running: bool,

    context: Option<Sdl>,
    canvas: Option<WindowCanvas>,
    entities: Option<Vec<&'a mut dyn Entity>>,
}

impl <'a>Default for Engine<'a> {
    fn default() -> Engine<'a> {
        Engine {
            loop_running: false,

            context: None,
            canvas: None,
            entities: None,
        }
    }
}

impl <'a>Engine<'a> {
    pub fn new(window_width: u32, window_height: u32) -> Result<Engine<'a>, String> {
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = video.window("Penis!", window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .build()
            .unwrap();

        let entities = Vec::<&mut dyn Entity>::new();

        let engine = Engine {
            context: Option::from(context),
            canvas: Option::from(canvas),
            entities: Option::from(entities),

            ..Engine::default()
        };

        Ok(engine)
    }

    pub fn add_entity<T: Entity + 'a>(&mut self, entity: &'a mut T) {
        let _ = self.entities.as_mut().unwrap().push(entity);
    }

    pub fn run_loop(&mut self) {
        self.loop_running = true;

        let context = self.context.as_ref().unwrap();
        let canvas = self.canvas.as_mut().unwrap();

        let mut event_queue = context.event_pump().unwrap();
        while self.loop_running {
            for event in event_queue.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        self.loop_running = false;
                    },

                    _=> {}
                }
            }

            for entity in self.entities.as_mut().unwrap() {
                entity.update(1).expect("entity update error");
            }

            canvas.set_draw_color(CLEAR_COLOUR);
            canvas.clear();

            for entity in self.entities.as_mut().unwrap() {
                entity.render(canvas).expect("entity render error");
            }

            canvas.present();
        }
    }
}