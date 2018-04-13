use camera::Camera;
use graphics::{Context, Graphics};


pub trait Drawable {
    fn draw<G: Graphics>(&self, camera: &Camera, ctx: &Context, g: &mut G);
}
