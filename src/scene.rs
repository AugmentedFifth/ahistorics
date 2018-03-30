use camera::Camera;
use drawable::Drawable;
use graphics::Context;
use opengl_graphics::GlGraphics;
use map_data::MapData;
use player::Player;


pub struct Scene {
    pub camera: Camera,
    pub map:    MapData,
    pub player: Player,
}


impl Scene {
    pub fn new(camera: Camera, map: MapData, player: Player) -> Self {
        Self { camera, map, player }
    }

    pub fn step(&mut self, dt: f64) {
        self.camera.pos.step(dt);
        self.player.pos.step(dt);
    }
}

impl Drawable for Scene {
    fn draw(&self, camera: &Camera, ctx: &Context, gl: &mut GlGraphics) {
        self.map.draw(camera, ctx, gl);
        self.player.draw(camera, ctx, gl);
    }
}
