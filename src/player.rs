pub struct Player {
    /// Position of player in terms of the underlying cubic coordinate grid.
    pos:                 CubePoint<i32>,
    /// Apparent position of the player, as viewed on screen.
    apparent_pos:        CubePoint<f32>,
    /// Previous apparent position that the player was at, only applicable when
    /// the player is animating.
    prev_apparent_pos:   CubePoint<f64>,
    /// Current progress of transition from `apparent_pos` to `pos`. `<= 0` is
    /// "just started", `>= 1` is "complete, no animation in progress".
    pos_state:           f64,
    /// Angle the player is oriented at.
    angle:               Angle,
    /// Angle that the player is displayed to be at.
    apparent_angle:      Angle,
    /// Previous apparent angle that the player was at, only applicable when
    /// the palyer's angle is animating.
    prev_apparent_angle: Angle,
    /// Current progress of transition from `apparent_angle` to `angle`. `<= 0`
    /// is "just started", `>= 1` is "complete, no animation in progress".
    angle_state:         f64,
}
