use sdl2::render::WindowCanvas;

pub trait Entity {
    fn update(&self, delta_time: i64) -> Result<(), String> {
        Ok(())
    }

    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        Ok(())
    }
}