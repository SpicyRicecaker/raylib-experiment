mod rotations;
mod input;

use tetromino::tetromino_type::TetrominoType;
use rotations::rotation_direction::RotationDirection;

use super::*;
use direction::*;
use rotations::*;

pub struct Universe {
    w: u32,
    h: u32,
    focused_tetromino: Tetromino,
    stagnant_tetrominos: Vec<Tetromino>,
    ticks: u32,
    tetromino_controls: TetrominoControls,
}

impl Universe {
    pub fn new(
        w: u32,
        h: u32,
        focused_tetromino: Tetromino,
        stagnant_tetrominos: Vec<Tetromino>,
        ticks: u32,
    ) -> Self {
        let tetromino_controls = TetrominoControls::new();
        Universe {
            w,
            h,
            focused_tetromino,
            stagnant_tetrominos,
            ticks,
            tetromino_controls,
        }
    }

    fn fall_focused(&mut self) {
        // Code that determines moving the pieces down
        let within_boundary = self
            .focused_tetromino
            .within_boundary(Tetromino::get_dxdy(Direction::Down));
        let mut collision = false;

        if within_boundary {
            collision = Tetromino::will_collide_all(
                &self.focused_tetromino,
                &self.stagnant_tetrominos,
                Tetromino::get_dxdy(Direction::Down),
            );
        }

        if !collision && within_boundary {
            self.focused_tetromino
                .move_by(Tetromino::get_dxdy(Direction::Down));
        } else {
            // Solidify the old current
            self.stagnant_tetrominos
                .push(self.focused_tetromino.clone());
            // Generate a new current
            self.focused_tetromino = TetrominoType::generate_tetromino_rand();
            // If it generates into a piece, game ova
            if Tetromino::will_collide_all(
                &self.focused_tetromino,
                &self.stagnant_tetrominos,
                [0, 0],
            ) {
                // Game over
            }
        }
    }

    /// Offset functions for tetromino rotation, need to apply for I and O
    pub fn apply_offset(&mut self) {
        // For I
        // For O
        // For any other piece (i.e., J, L, S, T, Z)

        // We start at 0 go to 90, 180, and 270 deg
        // let offset_indices = [0, 1, 2, 3];
    }

    fn rotate_focused(&mut self, rot_direction: RotationDirection) {
        let center_x = self.focused_tetromino.coords()[0].x;
        let center_y = self.focused_tetromino.coords()[0].y;

        let (next_index_diff, m) = match rot_direction {
            RotationDirection::Clockwise => (1, [[0, -1], [1, 0]]),
            RotationDirection::CounterClockwise => (-1, [[0, 1], [-1, 0]]),
        };

        for i in 1..self.focused_tetromino.coords().len() {
            let t = &mut self.focused_tetromino.coords_mut()[i];

            // Get the original coords by subtracting the origin
            // e.g. (1, 1), (1, 0), etc.
            let x = t.x as i32 - center_x as i32;
            let y = t.y as i32 - center_y as i32;
            // Rotate the coords 90 degrees to the left

            let f_x = x * m[0][0] + y * m[1][0];
            let f_y = x * m[0][1] + y * m[1][1];

            // Add the coords back
            t.x = (f_x + center_x as i32) as u32;
            t.y = (f_y + center_y as i32) as u32;
        }

        let offset_data = match self.focused_tetromino.tetromino_type() {
            TetrominoType::J
            | TetrominoType::L
            | TetrominoType::S
            | TetrominoType::T
            | TetrominoType::Z => &JLSTZ_OFFSET_DATA[..],
            TetrominoType::I => &I_OFFSET_DATA[..],
            TetrominoType::O => &O_OFFSET_DATA[..],
        };

        // Try all of the 5 test cases
        for test in offset_data {
            let current_set = test[*self.focused_tetromino().rotation_state().rn() as usize];
            let new_set = test[self
                .focused_tetromino()
                .rotation_state()
                .get_increment(next_index_diff) as usize];
            // Checkout <https://harddrop.com/wiki/SRS#How_Guideline_SRS_Really_Works> for more information on how the offset wallkicks are derived
            // Current - Next
            let dx_dy = [current_set[0] - new_set[0], current_set[1] - new_set[1]];

            // Test collisions
            // First make sure it's in boundaries
            if Tetromino::within_boundary(&self.focused_tetromino, dx_dy)
                && !Tetromino::will_collide_all(
                    &self.focused_tetromino,
                    &self.stagnant_tetrominos,
                    dx_dy,
                )
            {
                // Move tetrimino
                self.focused_tetromino_mut().move_by(dx_dy);
                // Update indice
                self.focused_tetromino_mut()
                    .rotation_state_mut()
                    .increment(next_index_diff);
                // Otherwise need to rotate back
                return;
            }
        }

        // Just rotate back if there is conflict, will show up as nothing happened
        // Good place to add sound as well
        self.rotate_focused(RotationDirection::flip(rot_direction));
    }

    pub fn tick(&mut self, rl: &RaylibHandle) {
        *self.ticks_mut() += 1;

        self.tetromino_controls.tick(rl);
        self.receive_key();

        // Literally just move current .y down
        // Falls at the rate of 6 per second

        if self.ticks() % 12 == 0 {
            self.fall_focused();
        }

        if *self.ticks() >= 60 {
            *self.ticks_mut() = 0;
        }

        let mut levels: HashMap<u32, u32> = HashMap::new();

        // Setup hash
        // We should probably store the hashmap, this way we won't have to update it every tick
        for tetromino in self.stagnant_tetrominos.iter() {
            for coord in tetromino.coords() {
                let e = levels.entry(coord.y).or_insert(0);
                *e += 1;
            }
        }

        let mut diff = [0; 20];

        let mut something_happened = false;
        // Scan hash
        for (level, width) in levels {
            // If the row is full
            if width == 10 {
                something_happened = true;
                // Query all tetrominos for level
                let mut i = 0;
                while i != self.stagnant_tetrominos.len() {
                    let mut j = 0;
                    while j != self.stagnant_tetrominos[i].coords().len() {
                        if self.stagnant_tetrominos[i].coords()[j].y == level {
                            self.stagnant_tetrominos[i].coords_mut().remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    // No memory leaks thank you
                    if self.stagnant_tetrominos[i].coords().is_empty() {
                        self.stagnant_tetrominos.remove(i);
                    } else {
                        i += 1;
                    }
                }
                // Everything above this width should then be incremented!
                Universe::change_arr_from_idx(&mut diff, level, 1);
            }
        }

        // Finally,if something happened try to move pieces down if they need to be moved
        // fk, we're iterating over stagnant tetrominos like 3 times. We honestly only need to really do it twice if we store the hashmap
        // If we implemented it with an array we would only need to iterate over the board once
        if something_happened {
            for i in 0..self.stagnant_tetrominos.len() {
                for j in 0..self.stagnant_tetrominos[i].coords().len() {
                    self.stagnant_tetrominos[i].coords_mut()[j].y -=
                        diff[self.stagnant_tetrominos[i].coords()[j].y as usize];
                }
            }
        }
    }

    pub fn change_arr_from_idx(arr: &mut [u32], idx: u32, diff: u32) {
        for num in arr.iter_mut().skip(idx as usize) {
            *num += diff;
        }
    }

    /// Renders the 10x20 grid that tetrominos spawn on oo
    fn render_grid(&self, d: &mut RaylibDrawHandle, config: &Config) {
        // Spawn tetrminoes at up to level 22
        // Only show 10x20 grid

        let dx = *config.actual_w() as u32 / 10;
        let dy = config.h() / 20;

        for x in (0..=10).into_iter() {
            // For every implement of x, draw from the ground to the ceiling
            let current_x = x * dx + *config.canvas_l() as u32;
            d.draw_line(
                current_x as i32,
                0,
                current_x as i32,
                *config.h() as i32,
                Color::from_hex("d4be98").unwrap(),
            );
        }
        for y in (0..=20).into_iter() {
            let current_y = y * dy;
            d.draw_line(
                *config.canvas_l() as i32,
                current_y as i32,
                *config.canvas_r() as i32,
                current_y as i32,
                Color::from_hex("d4be98").unwrap(),
            );
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, config: &Config) {
        // Render our current tetromino

        // Find the real world diff between each of the coords
        // let coords_dy = self.current().real().y() - self.current().center().y();
        // let coords_dx = self.current().real().x() - self.current().center().x();

        // Find the size of each cube
        self.focused_tetromino().render(d, config);

        // Draw every coord
        for tetromino in self.stagnant_tetrominos().iter() {
            tetromino.render(d, config);
        }

        // Render grid
        self.render_grid(d, config);
    }

    /// Get a reference to the universe's current.
    pub fn focused_tetromino(&self) -> &Tetromino {
        &self.focused_tetromino
    }

    /// Get a mutable reference to the universe's current.
    pub fn focused_tetromino_mut(&mut self) -> &mut Tetromino {
        &mut self.focused_tetromino
    }

    /// Get a reference to the universe's stagnant tetrominos.
    pub fn stagnant_tetrominos(&self) -> &Vec<Tetromino> {
        &self.stagnant_tetrominos
    }

    pub fn stagnant_tetrominos_mut(&mut self) -> &mut Vec<Tetromino> {
        &mut self.stagnant_tetrominos
    }

    /// Get a reference to the universe's ticks.
    pub fn ticks(&self) -> &u32 {
        &self.ticks
    }

    /// Get a mutable reference to the universe's ticks.
    pub fn ticks_mut(&mut self) -> &mut u32 {
        &mut self.ticks
    }
}

impl Default for Universe {
    fn default() -> Self {
        Universe::new(10, 40, TetrominoType::generate_tetromino_rand(), vec![], 0)
    }
}
