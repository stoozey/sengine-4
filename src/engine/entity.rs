use sdl2::render::WindowCanvas;

pub trait Entity {
    fn update(&mut self, delta_time: i64) -> Result<(), String> {
        Ok(())
    }

    fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        Ok(())
    }
}