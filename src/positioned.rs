use geometry::{Angle, CubePoint};


pub trait Positioned {
    fn unit_move(&mut self, forwards: bool);

    fn turn(&mut self, anticlockwise: bool);

    fn pos(&self) -> &CubePoint<f64>;

    fn angle(&self) -> Angle;
}
