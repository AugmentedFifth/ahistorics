use camera::Camera;
use graphics::Context;
use opengl_graphics::GlGraphics;


pub trait Drawable {
    fn draw(&self, camera: &Camera, ctx: &Context, gl: &mut GlGraphics);
}
