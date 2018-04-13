use geometry::{Angle, bezier2, cube_lerp, CubePoint};
use std::ops::{AddAssign, SubAssign};


#[derive(Debug, Clone)]
pub struct TransitionedGridPos {
    /// Total time that an animation phase of this position takes.
    anim_time:    f64,
    /// Real position in terms of the underlying cubic coordinate space.
    pos:          CubePoint<f64>,
    /// Position that is being moved towards.
    target_pos:   CubePoint<i32>,
    /// Previous position that this was at, only applicable when animating.
    prev_pos:     CubePoint<f64>,
    /// Current progress of transition from `pos` to `target_pos`. `<= 0` is
    /// "just started", `>= 1` is "complete, no animation in progress".
    pos_state:    f64,
    /// Angle of orientation.
    angle:        Angle,
    /// Angle that is being rotated to.
    target_angle: Angle,
    /// Previous angle that this was at, only applicable when animating.
    prev_angle:   Angle,
    /// Current progress of transition from `angle` to `target_angle`. `<= 0`
    /// is "just started", `>= 1` is "complete, no animation in progress".
    angle_state:  f64,
}


impl TransitionedGridPos {
    pub fn new(anim_time: f64, start_pos: CubePoint<f64>) -> Self {
        TransitionedGridPos {
            anim_time,
            pos:          start_pos,
            target_pos:   start_pos.map(|w| w as i32),
            prev_pos:     start_pos,
            pos_state:    1.0,
            angle:        Angle::new(0.0),
            target_angle: Angle::new(0.0),
            prev_angle:   Angle::new(0.0),
            angle_state:  0.0,
        }
    }

    pub fn pos(&self) -> &CubePoint<f64> {
        &self.pos
    }

    pub fn angle(&self) -> Angle {
        self.angle
    }

    pub fn target_pos(&self) -> &CubePoint<i32> {
        &self.target_pos
    }

    pub fn target_angle(&self) -> Angle {
        self.target_angle
    }

    pub fn set_target_pos(&mut self, target: CubePoint<i32>) {
        self.pos_state = 0.0;
        self.prev_pos = self.pos;
        self.target_pos = target;
    }

    pub fn inc_target_angle<A>(&mut self, increment: A)
        where Angle: AddAssign<A>
    {
        self.angle_state = 0.0;
        self.prev_angle = self.angle;
        self.target_angle += increment;
    }

    pub fn dec_target_angle<A>(&mut self, decrement: A)
        where Angle: SubAssign<A>
    {
        self.angle_state = 0.0;
        self.prev_angle = self.angle;
        self.target_angle -= decrement;
    }

    pub fn step(&mut self, dt: f64) {
        let target_pos_cast = self.target_pos.cast();

        if self.pos != target_pos_cast {
            if self.pos_state >= 1.0 {
                self.pos_state = dt / self.anim_time;

                self.pos = target_pos_cast;
                self.prev_pos = self.pos;
            } else {
                self.pos_state += dt / self.anim_time;

                let new_pos_progress = bezier2(
                    0.0,
                    0.75,
                    1.0,
                    self.pos_state.min(1.0)
                );

                self.pos = cube_lerp(
                    self.prev_pos,
                    self.target_pos,
                    new_pos_progress
                );
            }
        }

        if self.angle != self.target_angle {
            if self.angle_state >= 1.0 {
                self.angle_state = dt / self.anim_time;

                self.angle = self.target_angle;
                self.prev_angle = self.angle;
            } else {
                self.angle_state += dt / self.anim_time;

                let new_angle_progress = bezier2(
                    0.0,
                    0.75,
                    1.0,
                    self.angle_state.min(1.0)
                );

                self.angle = self.prev_angle.lerp(
                    &self.target_angle,
                    new_angle_progress
                );
            }
        }
    }
}
