mod tetris_input;
mod tetrominos;

use std::collections::HashMap;

use raylib::prelude::*;
use tetrominos::*;

use tetris_input::tetris::TetrominoControls;

/// Tetrominos of type J, L, S, T or Z each have 5 tests, accounting for each of the 4 indices, each with a cartesion coord
const JLSTZ_OFFSET_DATA: [[[i32; 2]; 4]; 5] = [
    [[0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [1, 0], [0, 0], [-1, 0]],
    [[0, 0], [1, -1], [0, 0], [-1, -1]],
    [[0, 0], [0, 2], [0, 0], [0, 2]],
    [[0, 0], [1, 2], [0, 0], [-1, 2]],
];

/// Tetromino of type  has 5 tests, each with 4 indices, each with a cartesion coord
const I_OFFSET_DATA: [[[i32; 2]; 4]; 5] = [
    [[0, 0], [-1, 0], [-1, 1], [0, 1]],
    [[-1, 0], [0, 0], [1, 1], [0, 1]],
    [[2, 0], [0, 0], [-2, 1], [0, -1]],
    [[-1, 0], [0, 1], [1, 0], [0, -1]],
    [[2, 0], [0, -2], [-2, 0], [0, 2]],
];
const O_OFFSET_DATA: [[[i32; 2]; 4]; 1] = [[[0, 0], [0, -1], [-1, -1], [-1, 0]]];

pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}
impl RotationDirection {
    // Gets reverse
    pub fn flip(direction: RotationDirection) -> RotationDirection {
        match direction {
            RotationDirection::Clockwise => RotationDirection::CounterClockwise,
            RotationDirection::CounterClockwise => RotationDirection::Clockwise,
        }
    }
}

pub struct Config {
    fps: u32,
    w: u32,
    h: u32,
    title: String,
    actual_w: f64,
    canvas_l: f64,
    canvas_r: f64,
}

impl Config {
    pub fn new(fps: u32, w: u32, h: u32, title: String) -> Self {
        let actual_w = w as f64 * (9_f64 / 32_f64);
        let canvas_l = (w as f64 - actual_w) / 2_f64;
        let canvas_r = canvas_l + actual_w;

        Config {
            fps,
            w,
            h,
            title,
            actual_w,
            canvas_l,
            canvas_r,
        }
    }

    /// Get a reference to the config's fps.
    pub fn fps(&self) -> &u32 {
        &self.fps
    }

    /// Get a reference to the config's title.
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get a reference to the config's h.
    pub fn h(&self) -> &u32 {
        &self.h
    }

    /// Get a reference to the config's w
    pub fn w(&self) -> &u32 {
        &self.w
    }

    /// Get a reference to the config's actual w.
    pub fn actual_w(&self) -> &f64 {
        &self.actual_w
    }

    /// Get a reference to the config's canvas l.
    pub fn canvas_l(&self) -> &f64 {
        &self.canvas_l
    }

    /// Get a reference to the config's canvas r.
    pub fn canvas_r(&self) -> &f64 {
        &self.canvas_r
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(60, 1600, 900, String::from("Tetris"))
    }
}

#[derive(Copy, Clone)]
pub enum Cell {
    Occupied = 1,
    Unoccupied = 0,
}

// The board for the tetris board
pub struct Universe {
    w: u32,
    h: u32,
    focused_tetromino: Tetromino,
    stagnant_tetrominos: Vec<Tetromino>,
    ticks: u32,
    tetromino_controls: TetrominoControls,
}

pub trait InputInterface {
    fn receive_key(&mut self);
}

impl InputInterface for Universe {
    fn receive_key(&mut self) {
        for key in self.tetromino_controls.get_queue() {
            match key {
                KeyboardKey::KEY_LEFT => {
                    if self.focused_tetromino.within_boundary(Direction::Left)
                        && !Tetromino::will_collide_all(
                            &self.focused_tetromino,
                            &self.stagnant_tetrominos,
                            Tetromino::get_dxdy(Direction::Left),
                        )
                    {
                        self.focused_tetromino.move_by(Tetromino::get_dxdy(Direction::Left))
                    }
                }
                KeyboardKey::KEY_RIGHT => {
                    if self.focused_tetromino.within_boundary(Direction::Right)
                        && !Tetromino::will_collide_all(
                            &self.focused_tetromino,
                            &self.stagnant_tetrominos,
                            Tetromino::get_dxdy(Direction::Right),
                        )
                    {
                        self.focused_tetromino.move_by(Tetromino::get_dxdy(Direction::Right))
                    }
                }
                KeyboardKey::KEY_DOWN => self.fall_focused(),
                KeyboardKey::KEY_Z => self.rotate_focused(RotationDirection::CounterClockwise),
                KeyboardKey::KEY_C => self.rotate_focused(RotationDirection::Clockwise),
                _ => {}
            }
        }
        self.tetromino_controls.clear_queue();
    }
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
        let within_boundary = self.focused_tetromino.within_boundary(Direction::Down);
        let mut collision = false;

        if within_boundary {
            collision = Tetromino::will_collide_all(
                &self.focused_tetromino,
                &self.stagnant_tetrominos,
                Tetromino::get_dxdy(Direction::Down),
            );
        }

        if !collision && within_boundary {
            self.focused_tetromino.move_by(Tetromino::get_dxdy(Direction::Down));
        } else {
            // Solidify the old current
            let temp = self.focused_tetromino.clone();
            // let temp = self.focused_tetromino.clone();
            self.stagnant_tetrominos.push(temp);
            // We need to generate a new current and solidify the old current
            self.focused_tetromino = TetrominoType::generate_tetromino_rand();
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

        let mut is_conflict = true;

        // Try all of the 5 test cases
        for i in 0..offset_data.len() {
            let current_set =
                offset_data[i][*self.focused_tetromino().rotation_state().rn() as usize];
            let new_set = offset_data[i][self
                .focused_tetromino()
                .rotation_state()
                .increment(next_index_diff) as usize];
            let dxdy = [new_set[0] - current_set[0], new_set[1] - current_set[1]];

            // Test collisions
            if !Tetromino::will_collide_all(
                &self.focused_tetromino,
                &self.stagnant_tetrominos,
                dxdy,
            ) {
                is_conflict = false;
                // Move tetrimino

                // Update indice

                // Otherwise need to rotate back
            }
        }

        // Just rotate back if there is conflict, will show up as nothing happened
        // Good place to add sound as well
        if is_conflict {
            self.rotate_focused(RotationDirection::flip(rot_direction));
        }
    }

    // self.focused_tetromino.rotation_state_mut().prev();

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_down() {
        let mut tetromino = Tetromino::spawn_tetrimno(
            vec![
                Coord::new(1, 0),
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(2, 0),
            ],
            Coord::new(5, 22),
            TetrominoType::T,
        );
        tetromino.move_by(Tetromino::get_dxdy(Direction::Down));

        let right_real_coords = vec![
            Coord { x: 4, y: 21 },
            Coord { x: 5, y: 22 },
            Coord { x: 5, y: 21 },
            Coord { x: 6, y: 21 },
        ];

        dbg!(&right_real_coords, tetromino.coords());

        for idx in (0..4).into_iter() {
            assert_eq!(right_real_coords.get(idx), tetromino.coords().get(idx))
        }
    }
    #[test]
    fn test_move_down_3() {
        let mut tetromino = Tetromino::spawn_tetrimno(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            Coord::new(5, 22),
            TetrominoType::T,
        );
        tetromino.move_by(Tetromino::get_dxdy(Direction::Left));
        tetromino.move_by(Tetromino::get_dxdy(Direction::Left));
        tetromino.move_by(Tetromino::get_dxdy(Direction::Left));

        let right_real_coords = vec![
            Coord { x: 4, y: 19 },
            Coord { x: 5, y: 20 },
            Coord { x: 5, y: 19 },
            Coord { x: 6, y: 19 },
        ];

        dbg!(&right_real_coords, tetromino.coords());

        for idx in (0..4).into_iter() {
            assert_eq!(right_real_coords.get(idx), tetromino.coords().get(idx))
        }
    }

    #[test]
    fn test_increment_arr() {
        let mut arr = [0_u32; 5];
        let test = [0, 1, 1, 1, 1];
        Universe::change_arr_from_idx(&mut arr, 1, 1);
        assert_eq!(arr, test);
        dbg!(arr);
    }
}
