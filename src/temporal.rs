pub trait Temporal {
    fn step(&mut self, dt: f64);
}
