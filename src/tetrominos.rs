use rand::{distributions::Standard, prelude::Distribution, Rng};
use raylib::prelude::*;
use std::collections::HashSet;

use crate::Config;

#[derive(Clone)]
pub struct CircularNum {
    rn: u32,
    max: u32,
}

/// Based on
/// ```rust
/// fn pos_neg_modulus(&mut self, x: u32, m: u32) -> u32 {
///     (x % m + m) % m
/// }
/// ```
impl CircularNum {
    /// Takes in any dx
    pub fn get_increment(&self, dx: i32) -> u32 {
        // DEBUG
        // self.rn = ((self.rn as i32 + dx + self.max as i32) % self.max as i32) as u32;
        // &self.rn
        ((self.rn as i32 + dx + self.max as i32) % self.max as i32) as u32
    }

    /// Actually increments
    pub fn increment(&mut self, dx: i32) {
        // DEBUG
        // self.rn = ((self.rn as i32 + dx + self.max as i32) % self.max as i32) as u32;
        // &self.rn
        self.rn = ((self.rn as i32 + dx + self.max as i32) % self.max as i32) as u32;
    }


    /// Get a reference to the circular num's rn.
    pub fn rn(&self) -> &u32 {
        &self.rn
    }
}

impl Default for CircularNum {
    fn default() -> Self {
        CircularNum { rn: 0, max: 4 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Self {
        Coord { x, y }
    }
    pub fn x(&self) -> &u32 {
        &self.x
    }
    pub fn mut_x(&mut self) -> &mut u32 {
        &mut self.x
    }
    pub fn y(&self) -> &u32 {
        &self.y
    }
    pub fn mut_y(&mut self) -> &mut u32 {
        &mut self.y
    }
}

impl Default for Coord {
    fn default() -> Self {
        Coord { x: 0, y: 0 }
    }
}

pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

/// Built off tetromino coords
#[derive(Clone)]
pub struct Tetromino {
    coords: Vec<Coord>,
    tetromino_type: TetrominoType,
    rotation_state: CircularNum,
}

impl Tetromino {
    /// Generates a tetromino, given a set of coords, a type
    /// The center of the tetromino, as well as the location it should be spawned in
    pub fn spawn_tetrimno(
        // List of coords
        reference_coords: Vec<Coord>,
        // Real center is where to spawn the tetromino
        spawn_coords: Coord,
        // Type of tetromino
        tetromino_type: TetrominoType,
    ) -> Tetromino {
        // Generate real coords from reference coords
        let coords = reference_coords
            .iter()
            .map(|coord| {
                let dx: i32 = coord.x as i32 - reference_coords[0].x as i32;
                let dy: i32 = coord.y as i32 - reference_coords[0].y as i32;
                Coord::new(
                    (spawn_coords.x as i32 + dx) as u32,
                    (spawn_coords.y as i32 + dy) as u32,
                )
            })
            .collect();

        Tetromino {
            coords,
            tetromino_type,
            rotation_state: CircularNum::default(),
        }
    }

    /// Gives true pixel value,
    /// since graphics use 4th quadrant instead of 1st
    pub fn reversed_coord_y(canvas_y: u32, coord_y: u32, dy: u32) -> i32 {
        (canvas_y - (coord_y * dy)) as i32
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, config: &Config) {
        let dy = config.h() / 20;
        let dx = *config.actual_w() as u32 / 10;

        // For every coord in the tetromino (4 coords in total)
        for (idx, coord) in self.coords.iter().enumerate() {
            if coord.y >= 20 {
                continue;
            }
            // Figure out what this means in terms of real coords
            d.draw_rectangle(
                (config.canvas_l as u32 + coord.x * dx) as i32,
                (config.h - (coord.y + 1) * dy) as i32,
                dx as i32,
                dy as i32,
                if idx == 0 {
                    Color::from_hex("ea6962").unwrap()
                } else {
                    Color::from_hex("d4be98").unwrap()
                },
            )
        }
    }

    pub fn within_boundary(&self, dx_dy: [i32; 2]) -> bool {
        for coord in self.coords.iter() {
            if !(0..10).contains(&(coord.x as i32 + dx_dy[0])) || !(0..24).contains(&(coord.y as i32 + dx_dy[1])) {
                return false;
            }
        }
        true
    }

    pub fn get_dxdy(direction: Direction) -> [i32; 2] {
        match direction {
            Direction::Down => [0, -1],
            Direction::Up => [0, 1],
            Direction::Left => [-1, 0],
            Direction::Right => [1, 0],
        }
    }

    pub fn will_collide_all(
        t: &Tetromino,
        stagnant_tetrominos: &[Tetromino],
        dxdy: [i32; 2],
    ) -> bool {
        for stagnant_tetromino in stagnant_tetrominos {
            if Tetromino::will_collide(t, stagnant_tetromino, dxdy[0], dxdy[1]) {
                return true;
            }
        }
        false
    }

    pub fn will_collide(f: &Tetromino, s: &Tetromino, dx: i32, dy: i32) -> bool {
        let mut coords: HashSet<Coord> = HashSet::new();
        for f_coord in f.coords.iter() {
            coords.insert(Coord {
                x: (f_coord.x as i32 + dx) as u32,
                y: (f_coord.y as i32 + dy) as u32,
            });
        }
        for s_coord in s.coords.iter() {
            if coords.contains(s_coord) {
                return true;
            }
        }
        false
    }

    pub fn move_by(&mut self, dx_dy: [i32; 2]) {
        // Moves all real coords
        self.coords.iter_mut().for_each(|c| {
            c.x = (c.x as i32 + dx_dy[0]) as u32;
            c.y = (c.y as i32 + dx_dy[1]) as u32;
        });
    }
}

impl Tetromino {
    /// Get a mutable reference to the tetromino's rotation state.
    pub fn rotation_state_mut(&mut self) -> &mut CircularNum {
        &mut self.rotation_state
    }

    /// Get a reference to the tetromino's tetromino type.
    pub fn tetromino_type(&self) -> &TetrominoType {
        &self.tetromino_type
    }

    /// Get a reference to the tetromino's rotation state.
    pub fn rotation_state(&self) -> &CircularNum {
        &self.rotation_state
    }

    pub fn coords(&self) -> &Vec<Coord> {
        &self.coords
    }
    pub fn coords_mut(&mut self) -> &mut Vec<Coord> {
        &mut self.coords
    }
}

#[derive(Clone, Copy)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Distribution<TetrominoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
        match rng.gen_range(0..7) {
            0 => TetrominoType::I,
            1 => TetrominoType::J,
            2 => TetrominoType::L,
            3 => TetrominoType::O,
            4 => TetrominoType::S,
            5 => TetrominoType::T,
            6 => TetrominoType::Z,
            _ => {
                panic!()
            }
        }
    }
}

impl TetrominoType {
    pub fn generate_tetromino_rand() -> Tetromino {
        TetrominoType::generate_tetromino_from_type(rand::random())
    }
    /// Function that takes in a tetromino type and returns a spawned tetromino
    /// Important to realize that the first index of reference coords are the center of the tetromino
    pub fn generate_tetromino_from_type(tetromino_type: TetrominoType) -> Tetromino {
        match tetromino_type {
            TetrominoType::I => Tetromino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(2, 0),
                    Coord::new(3, 0),
                ],
                Coord::new(5, 22),
                TetrominoType::I,
            ),
            TetrominoType::J => Tetromino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(2, 0),
                    Coord::new(0, 1),
                ],
                Coord::new(5, 22),
                TetrominoType::J,
            ),
            TetrominoType::L => Tetromino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(2, 0),
                    Coord::new(2, 1),
                ],
                Coord::new(5, 22),
                TetrominoType::L,
            ),
            TetrominoType::O => Tetromino::spawn_tetrimno(
                vec![
                    Coord::new(0, 0),
                    Coord::new(1, 0),
                    Coord::new(0, 1),
                    Coord::new(1, 1),
                ],
                Coord::new(5, 22),
                TetrominoType::O,
            ),
            TetrominoType::S => Tetromino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(1, 1),
                    Coord::new(2, 1),
                ],
                Coord::new(5, 22),
                TetrominoType::S,
            ),
            TetrominoType::T => Tetromino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(1, 1),
                    Coord::new(2, 0),
                ],
                Coord::new(5, 22),
                TetrominoType::T,
            ),
            TetrominoType::Z => Tetromino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(2, 0),
                    Coord::new(0, 1),
                    Coord::new(1, 1),
                ],
                Coord::new(5, 22),
                TetrominoType::Z,
            ),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_t_tetromino_spawn() {
        let tetromino = Tetromino::spawn_tetrimno(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            Coord::new(5, 22),
            TetrominoType::T,
        );
        let right_real_coords = vec![
            Coord { x: 4, y: 22 },
            Coord { x: 5, y: 23 },
            Coord { x: 5, y: 22 },
            Coord { x: 6, y: 22 },
        ];
        dbg!(&right_real_coords, tetromino.coords());

        for idx in (0..4).into_iter() {
            assert_eq!(right_real_coords.get(idx), tetromino.coords().get(idx))
        }
    }
    #[test]
    fn test_boundary_positive() {
        let tetromino = Tetromino::spawn_tetrimno(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            Coord::new(5, 10),
            TetrominoType::T,
        );
        assert!(tetromino.within_boundary(Tetromino::get_dxdy(Direction::Down)));
    }

    #[test]
    fn test_boundary_negative() {
        let tetromino = Tetromino::spawn_tetrimno(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            Coord::new(5, 0),
            TetrominoType::T,
        );
        assert!(tetromino.within_boundary(Tetromino::get_dxdy(Direction::Down)));
    }
}
