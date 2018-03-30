#![deny(missing_docs)]

#![feature(collection_placement)]
#![feature(placement_in_syntax)]
// #![feature(slice_patterns)]

//! A 2D action RPG, written in pure Rust

mod camera;
mod controls;
mod geometry;
mod map_data;
mod matrix;
mod player;
mod positioned;
mod settings;
mod transitioned_grid_pos;

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
use geometry::{CubePoint, cube_to_real, HEXAGON_POLY};
use graphics::{
    math::add,
    rectangle::{Border, Rectangle, Shape},
    polygon::Polygon,
};
use map_data::{Hex, simulated_map_data};
use matrix::{m, rot, scale_uni, trans};
use sdl2_window::Sdl2Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{Events, EventSettings},
    input::{
        Button,
        PressEvent,
        ReleaseEvent,
        RenderEvent,
        UpdateEvent,
    },
    window::WindowSettings,
};
use player::Player;
use settings::Settings;


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

    // Retrieve settings.
    let settings =
        Settings::get_from_recur("./ahistorics_settings.toml").unwrap();

    // Initialize player.
    let mut player = Player::new(0.25, CubePoint::new(0.0, 0.0, 0.0));

    // Initializing some constants for draw testing. Will be moved/removed.
    let hex_scaled_height = 12.0;
    let mut camera = Camera::new(0.4, CubePoint::new(0.0, 0.0, 0.0));

    let side_len = 24;
    let map_data = simulated_map_data(side_len);
    let scale_factor = f64::from(WINDOW_HEIGHT) / hex_scaled_height;
    let spacing_factor = 0.875;

    let new_hex = Polygon::new(settings.colors.foreground_color);

    let player_square = Rectangle::new(settings.colors.player_color)
        .shape(Shape::Bevel(1.0))
        .border(Border {
            color: settings.colors.player_outline_color,
            radius: 1.0,
        });

    // Initialize controls to handle keypresses, clicks, etc.
    let mut controls = Controls::new();

    // The game's main loop.
    while let Some(event) = events.next(&mut window) {
        // If this event is a "render" event.
        if let Some(render_args) = event.render_args() {
            // Start drawing to the screen.
            gl_graphics.draw(render_args.viewport(), |ctx, gl| {
                // Clear the entire window.
                graphics::clear(settings.colors.background_color, gl);

                // Draw the scene.
                let cam_rotation = rot(camera.pos.angle().radians());
                for (hex, x, y) in map_data.iter() {
                    if hex == &Hex::Blank {
                        continue;
                    }

                    let q = x as i32;
                    let r = y as i32 - q / 2;
                    let abs_cube_pos = CubePoint::from_q_r(q, r).cast();

                    let tile_minus_cam = abs_cube_pos - *camera.pos.pos();

                    let pos = add(
                        cam_rotation.vec_mul(cube_to_real(
                            tile_minus_cam,
                            scale_factor
                        )),
                        [HALF_WINDOW_WIDTH, HALF_WINDOW_HEIGHT]
                    );

                    if pos[0] > -scale_factor                          &&
                       pos[0] < f64::from(WINDOW_WIDTH) + scale_factor &&
                       pos[1] > -scale_factor                          &&
                       pos[1] < f64::from(WINDOW_HEIGHT) + scale_factor
                    {
                        let depth_factor = if let Hex::Tile(depth) = *hex {
                            1.0 + f64::from(depth) / 16.0
                        } else {
                            1.0
                        };

                        let transform =
                            cam_rotation *
                            scale_uni(
                                scale_factor *
                                (spacing_factor * depth_factor).min(0.975)
                            ) *
                            trans(pos) *
                            m(ctx.transform);

                        new_hex.draw(
                            HEXAGON_POLY,
                            &ctx.draw_state,
                            transform.repr,
                            gl
                        );
                    }
                }

                let player_abs_pos = *player.pos.pos();
                let player_minus_cam = player_abs_pos - *camera.pos.pos();
                let player_disp = add(
                    cam_rotation.vec_mul(cube_to_real(
                        player_minus_cam,
                        scale_factor
                    )),
                    [HALF_WINDOW_WIDTH, HALF_WINDOW_HEIGHT]
                );

                let player_abs_angle = player.pos.angle();
                let player_ang_minus_cam =
                    player_abs_angle - camera.pos.angle();

                let player_trans =
                    rot(-player_ang_minus_cam.radians()) *
                    trans(player_disp) *
                    m(ctx.transform);

                player_square.draw(
                    [-scale_factor / 4.0, -scale_factor / 4.0,
                      scale_factor / 2.0,  scale_factor / 2.0],
                    &ctx.draw_state,
                    player_trans.repr,
                    gl
                );
            });
        }

        // If this event is an "update" event.
        if let Some(update_args) = event.update_args() {
            // Step forward the physics logic.
            //scene.physics_update(update_args.dt);
            camera.pos.step(update_args.dt);
            player.pos.step(update_args.dt);
        }

        // If this event is a keyboard key being pressed down.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            controls.press(key, &mut camera, &mut player);
        }

        // If this event is a keyboard key being released.
        if let Some(Button::Keyboard(key)) = event.release_args() {
            controls.release(&key);
        }
    }
}
