use camera::Camera;
use drawable::Drawable;
use geometry::{Angle, cube_dir, cube_to_real, CubePoint, Dir};
use graphics::{
    Context,
    Graphics,
    math::add,
    rectangle::{Border, Rectangle, Shape},
};
use matrix::{m, rot, trans};
use positioned::Positioned;
use settings::Settings;
use std::f64::consts::FRAC_PI_3;
use temporal::Temporal;
use transitioned_grid_pos::TransitionedGridPos;
use window::{HALF_WINDOW_HEIGHT, HALF_WINDOW_WIDTH, WINDOW_HEIGHT};


#[derive(Clone)]
pub struct Player {
    /// Position of player in terms of the underlying cubic coordinate system.
    pos:  TransitionedGridPos,
    rect: Rectangle,
}


impl Player {
    pub fn new(anim_time: f64,
               start_pos: CubePoint<f64>,
               settings:  &Settings) -> Self
    {
        Self {
            pos:  TransitionedGridPos::new(anim_time, start_pos),
            rect: Rectangle::new(settings.colors.player_color)
                      .shape(Shape::Bevel(1.0))
                      .border(Border {
                          color:  settings.colors.player_outline_color,
                          radius: 1.0,
                      }),
        }
    }
}

impl Positioned for Player {
    fn unit_move(&mut self, forwards: bool) {
        let target_angle = self.pos.target_angle();
        let turns = (target_angle.radians() / FRAC_PI_3)
            .round() as u8 % 6;

        let target_pos = *self.pos.target_pos();
        let target_dir = cube_dir(match turns {
            0 => Dir::Up,
            1 => Dir::UpLeft,
            2 => Dir::DownLeft,
            3 => Dir::Down,
            4 => Dir::DownRight,
            5 => Dir::UpRight,
            _ => unreachable!(),
        });
        let new_target_pos = if forwards {
            target_pos + target_dir
        } else {
            target_pos - target_dir
        };

        self.pos.set_target_pos(new_target_pos);
    }

    fn turn(&mut self, anticlockwise: bool) {
        if anticlockwise {
            self.pos.inc_target_angle(FRAC_PI_3);
        } else {
            self.pos.dec_target_angle(FRAC_PI_3);
        }
    }

    fn pos(&self) -> &CubePoint<f64> {
        self.pos.pos()
    }

    fn angle(&self) -> Angle {
        self.pos.angle()
    }
}

impl Drawable for Player {
    fn draw<G: Graphics>(&self, camera: &Camera, ctx: &Context, g: &mut G) {
        // TODO: `hex_scaled_height` should be dynamic state.
        let hex_scaled_height = 12.0;
        let scale_factor = f64::from(WINDOW_HEIGHT) / hex_scaled_height;

        let cam_rotation = rot(camera.angle().radians());

        let player_abs_pos = *self.pos();
        let player_minus_cam = player_abs_pos - *camera.pos();
        let player_disp = add(
            cam_rotation.vec_mul(cube_to_real(
                player_minus_cam,
                scale_factor,
            )),
            [HALF_WINDOW_WIDTH, HALF_WINDOW_HEIGHT],
        );

        let player_abs_angle = self.angle();
        let player_ang_minus_cam = player_abs_angle - camera.angle();

        let player_trans =
            rot(-player_ang_minus_cam.radians()) *
            trans(player_disp) *
            m(ctx.transform);

        self.rect.draw(
            [-scale_factor / 4.0, -scale_factor / 4.0,
              scale_factor / 2.0,  scale_factor / 2.0],
            &ctx.draw_state,
            player_trans.repr,
            g,
        );
    }
}

impl Temporal for Player {
    fn step(&mut self, dt: f64) {
        self.pos.step(dt);
    }
}
