#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![deny(missing_docs)]

#![feature(collection_placement)]
#![feature(placement_in_syntax)]
#![feature(slice_patterns)]

//! A 2D action RPG, written in pure Rust

mod camera;
mod controls;
mod geometry;
mod map_data;
mod matrix;

extern crate sdl2_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate vecmath;

use camera::Camera;

use controls::Controls;

use geometry::{AxialPoint, axial_to_real, HEXAGON_POLY};

use sdl2_window::Sdl2Window;

use graphics::polygon::Polygon;

use map_data::{Hex, simulated_map_data};

use matrix::{m, rot, scale_uni, trans};

use opengl_graphics::{GlGraphics, OpenGL};

use piston::event_loop::{Events, EventSettings};
use piston::input::{
    Button,
    PressEvent,
    ReleaseEvent,
    RenderEvent,
    UpdateEvent,
};
use piston::window::WindowSettings;


const WINDOW_WIDTH:  u32 = 1_366;
const WINDOW_HEIGHT: u32 = 768;

const HALF_WINDOW_WIDTH:  f64 = WINDOW_WIDTH  as f64 / 2.0;
const HALF_WINDOW_HEIGHT: f64 = WINDOW_HEIGHT as f64 / 2.0;


/// Entry point for the program.
fn main() {
    // Constants.
    const OPENGL: OpenGL = OpenGL::V4_5;

    // Setting up the window.
    let window_settings =
        WindowSettings::new("ahistorics", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .vsync(true)
            .samples(4)
            .opengl(OPENGL);

    let mut window: Sdl2Window =
        window_settings
            .build()
            .expect("Could not create window!");

    // Setting up events to loop over.
    let event_settings = EventSettings {
        ups: 60,
        ..EventSettings::new()
    };
    let mut events = Events::new(event_settings);

    // Initialize graphics backend that we can call `.draw()` on.
    let mut gl_graphics = GlGraphics::new(OPENGL);

    // Initializing some coostants for draw testing. Will be moved/removed.
    let hex_scaled_height = 12.0;
    let hex_scaled_width =
        hex_scaled_height *
            WINDOW_WIDTH as f64 / WINDOW_HEIGHT as f64;
    let mut camera = Camera::new(
        hex_scaled_width,
        hex_scaled_height,
        0.375,
        [6.0, 7.0]
    );

    let side_len = 24;
    let map_data = simulated_map_data(side_len);
    let scale_factor = WINDOW_HEIGHT as f64 / hex_scaled_height;
    let spacing_factor = 0.875;

    let new_hex = Polygon::new([0.875, 0.875, 0.875, 1.0]);

    let mut controls = Controls::new();

    // The game's main loop.
    while let Some(event) = events.next(&mut window) {
        // If this event is a "render" event.
        if let Some(render_args) = event.render_args() {
            // Start drawing to the screen.
            gl_graphics.draw(render_args.viewport(), |ctx, gl| {
                // Clear the entire window.
                graphics::clear([0.0625, 0.0625, 0.0625, 1.0], gl);

                // Draw the scene.
                let rotation = rot(camera.angle());
                camera.draw(|x, y| {
                    let hex = if let Some(h) = map_data.get_rect(x, y) {
                        h
                    } else {
                        eprintln!(
                            "indexed into nonexistent map data: ({}, {}) \
                             into data of dimensions ({}, {})",
                            x,
                            y,
                            map_data.cols(),
                            map_data.rows()
                        );

                        return;
                    };

                    if hex == &Hex::Blank {
                        return;
                    }

                    let r = y as i32;
                    let q = x as i32 - r / 2;
                    let pos = axial_to_real(
                        AxialPoint::new(q, r),
                        scale_factor
                    );

                    let transform =
                        rotation *
                        scale_uni(scale_factor * spacing_factor) *
                        trans(pos) *
                        m(ctx.transform);

                    new_hex.draw(
                        HEXAGON_POLY,
                        &ctx.draw_state,
                        transform.repr,
                        gl
                    );
                }, side_len, side_len);

                // Testingg
                //camera.inc_angle(0.01);

                //scene.draw(&ctx, gl);
            });
        }

        // If this event is an "update" event.
        if let Some(update_args) = event.update_args() {
            // Step forward the physics logic.
            //scene.physics_update(update_args.dt);
            camera.step(update_args.dt);
        }

        // If this event is a keyboard key being pressed down.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            controls.press(key, &mut camera);
        }

        // If this event is a keyboard key being released.
        if let Some(Button::Keyboard(key)) = event.release_args() {
            controls.release(key);
        }
    }
}
