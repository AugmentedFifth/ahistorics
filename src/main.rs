#![allow(missing_docs)]

//! A 2D action RPG, written in pure Rust

mod camera;
mod controls;
mod draw;
mod drawable;
mod geometry;
mod map_data;
mod matrix;
mod player;
mod positioned;
mod scene;
mod settings;
mod transitioned_grid_pos;
mod window;

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate fnv;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate sdl2_window;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate vecmath;

use camera::Camera;
use controls::Controls;
use failure::Error;
use geometry::CubePoint;
use map_data::simulated_map_data;
use opengl_graphics::GlGraphics;
use piston::{
    event_loop::Events,
    input::{
        Button,
        PressEvent,
        ReleaseEvent,
        RenderEvent,
        UpdateEvent,
    },
    window::Window,
};
use player::Player;
use scene::Scene;
use settings::Settings;


/// Entry point for the program.
fn main() {
    if let Err(e) = main_() {
        eprintln!("Something went wrong:");
        eprintln!("    {}.", e);

        std::process::exit(1);
    }
}

/// Real entry point for the program.
fn main_() -> Result<(), Error> {
    let settings = Settings::get_from_recur("./ahistorics_settings.toml")?;
    let player = Player::new(0.25, CubePoint::new(0.0, 0.0, 0.0), &settings);
    let map = simulated_map_data(24, settings.colors.foreground_color)?;
    let camera = Camera::new(0.4, CubePoint::new(0.0, 0.0, 0.0));
    let scene = Scene::new(camera, map, player);

    main_loop(window::events(), window::init()?, scene, settings)
}

/// The main game loop.
fn main_loop<W>(mut events: Events,
                mut window: W,
                mut scene:  Scene,
                settings:   Settings) -> Result<(), Error>
    where W: Window
{
    // Initialize graphics backend that we can call `.draw()` on.
    let mut gl_graphics = GlGraphics::new(window::OPENGL);

    // Initialize controls to handle keypresses, clicks, etc.
    let mut controls = Controls::new();

    while let Some(event) = events.next(&mut window) {
        // If this event is a "render" event.
        if let Some(render_args) = event.render_args() {
            draw::draw(&mut gl_graphics, &render_args, &settings, &scene);
        }

        // If this event is an "update" event.
        if let Some(update_args) = event.update_args() {
            scene.step(update_args.dt);
        }

        // If this event is a keyboard key being pressed down.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            controls.press(key, &mut scene.camera, &mut scene.player);
        }

        // If this event is a keyboard key being released.
        if let Some(Button::Keyboard(key)) = event.release_args() {
            controls.release(&key);
        }
    }

    Ok(())
}
