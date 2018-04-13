use failure::{err_msg, Error};
use opengl_graphics::GlGraphics;
use piston::{
    event_loop::{Events, EventSettings},
    window::{Window, WindowSettings},
};
use sdl2_window::{OpenGL, Sdl2Window};


pub const WINDOW_WIDTH:  u32 = 1_366;
pub const WINDOW_HEIGHT: u32 = 768;

pub const HALF_WINDOW_WIDTH:  f64 = WINDOW_WIDTH  as f64 / 2.0;
pub const HALF_WINDOW_HEIGHT: f64 = WINDOW_HEIGHT as f64 / 2.0;

pub const OPENGL: OpenGL = OpenGL::V4_5;


/// Sets up the window.
pub fn init() -> Result<Sdl2Window, Error> {
    let window_settings =
        WindowSettings::new("ahistorics", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .vsync(true)
            .samples(4)
            .opengl(OPENGL);

    window_settings.build().map_err(err_msg)
}

/// Sets up events for the window.
pub fn events() -> Events {
    let event_settings = EventSettings {
        ups: 60,
        ..EventSettings::new()
    };

    Events::new(event_settings)
}

/// Initializes graphics backend that one can call `.draw(...)` on.
pub fn graphics_init<W: Window>(_window: &mut W) -> GlGraphics {
    GlGraphics::new(OPENGL)
}
