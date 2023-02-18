use sdl2::pixels::Color;
use sdl2::event::Event;

mod engine;

fn main() -> Result<(), String> {
    let mut engine = engine::Engine::create(1280, 720)?;
    engine.run_loop();

    Ok(())
}