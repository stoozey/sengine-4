mod engine;

use std::borrow::BorrowMut;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use engine::Engine;
use engine::entity::Entity;

struct PenisMan {
    x: f32,
    y: f32,
}

impl Entity for PenisMan {
    fn update(&mut self, delta_time: i64) -> Result<(), String> {
        self.x += 0.01;
        self.y += 0.01;
        Ok(())
    }

    fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RED);
        let rect = Rect::new(self.x as i32, self.y as i32, 32, 32);
        canvas.draw_rect(rect)?;
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let mut engine = engine::Engine::new(1280, 720)?;
    let mut penis_man = PenisMan{ x: 0.0, y: 0.0 };
    engine.add_entity(&mut penis_man);

    engine.run_loop();

    Ok(())
}