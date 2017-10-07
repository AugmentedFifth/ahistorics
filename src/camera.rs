use transitioned_grid_pos::TransitionedGridPos;


pub struct Camera {
    pos: TransitionedGridPos,
}


impl Camera {
    pub fn new(anim_time: f64, start_pos: CubePoint<f64>) -> Self {
        Camera {
            pos: TransitionedGridPos::new(anim_time, start_pos),
        }
    }

    pub fn pos(&self) -> &TransitionedGridPos {
        &self.pos
    }

    pub fn draw<F: FnMut(usize, usize) -> ()>(
        &self,
        mut draw_fn: F,
        cols:        usize,
        rows:        usize
    ) {
        for x in 0..cols {
            for y in 0..rows {
                draw_fn(x, y);
            }
        }
    }
}
