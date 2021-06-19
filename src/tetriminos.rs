use rand::{distributions::Standard, prelude::Distribution, Rng};
use raylib::prelude::*;
use std::collections::HashSet;

use crate::Config;

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

/// Built off tetrimino coords
#[derive(Clone)]
pub struct Tetrimino {
    coords: Vec<Coord>,
    tetrimino_type: TetriminoType,
}

impl Tetrimino {
    pub fn coords(&self) -> &Vec<Coord> {
        &self.coords
    }
    pub fn coords_mut(&mut self) -> &mut Vec<Coord> {
        &mut self.coords
    }

    /// Generates a tetrimino, given a set of coords, a type
    /// The center of the tetrimino, as well as the location it should be spawned in
    pub fn spawn_tetrimno(
        // List of coords
        reference_coords: Vec<Coord>,
        // Real center is where to spawn the tetrimino
        spawn_coords: Coord,
        // Type of tetrimino
        tetrimino_type: TetriminoType,
    ) -> Tetrimino {
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

        Tetrimino {
            coords,
            tetrimino_type,
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

        // For every coord in the tetrimino (4 coords in total)
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
    // Moves all real coords
    pub fn move_down(&mut self) {
        self.coords.iter_mut().for_each(|c| c.y -= 1);
    }

    pub fn within_boundary(&self, direction: Direction) -> bool {
        match direction {
            Direction::Up => false,
            Direction::Down => {
                for coord in self.coords.iter() {
                    if coord.y == 0 {
                        return false;
                    }
                }
                true
            }
            Direction::Left => {
                for coord in self.coords.iter() {
                    if coord.x == 0 {
                        return false;
                    }
                }
                true
            }
            Direction::Right => {
                for coord in self.coords.iter() {
                    if coord.x + 1 >= 10 {
                        return false;
                    }
                }
                true
            }
        }
    }

    pub fn will_collide_all(
        t: &Tetrimino,
        stagnant_tetriminos: &[Tetrimino],
        direction: Direction,
    ) -> bool {
        let (dx, dy) = match direction {
            Direction::Down => (0, -1),
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for stagnant_tetrimino in stagnant_tetriminos {
            if Tetrimino::will_collide(t, stagnant_tetrimino, dx, dy) {
                return true;
            }
        }
        false
    }

    pub fn will_collide(f: &Tetrimino, s: &Tetrimino, dx: i32, dy: i32) -> bool {
        let mut coords: HashSet<Coord> = HashSet::new();
        for f_coord in f.coords.iter() {
            coords.insert(Coord {
                x: (*f_coord.x() as i32 + dx) as u32,
                y: (*f_coord.y() as i32 + dy) as u32,
            });
        }
        for s_coord in s.coords.iter() {
            if coords.contains(s_coord) {
                return true;
            }
        }
        false
    }

    pub fn move_left(&mut self) {
        for coord in self.coords_mut() {
            coord.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        for coord in self.coords_mut() {
            *coord.mut_x() += 1;
        }
    }
}

#[derive(Clone, Copy)]
pub enum TetriminoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Distribution<TetriminoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetriminoType {
        match rng.gen_range(0..7) {
            0 => TetriminoType::I,
            1 => TetriminoType::J,
            2 => TetriminoType::L,
            3 => TetriminoType::O,
            4 => TetriminoType::S,
            5 => TetriminoType::T,
            6 => TetriminoType::Z,
            _ => {
                panic!()
            }
        }
    }
}

impl TetriminoType {
    pub fn generate_tetrimino_rand() -> Tetrimino {
        TetriminoType::generate_tetrimino_from_type(rand::random())
    }
    /// Function that takes in a tetrimino type and returns a spawned tetrimino
    /// Important to realize that the first index of reference coords are the center of the tetrimino
    pub fn generate_tetrimino_from_type(tetrimino_type: TetriminoType) -> Tetrimino {
        match tetrimino_type {
            TetriminoType::I => Tetrimino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(2, 0),
                    Coord::new(3, 0),
                ],
                Coord::new(5, 22),
                TetriminoType::I,
            ),
            TetriminoType::J => Tetrimino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(2, 0),
                    Coord::new(0, 1),
                ],
                Coord::new(5, 22),
                TetriminoType::J,
            ),
            TetriminoType::L => Tetrimino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(2, 0),
                    Coord::new(2, 1),
                ],
                Coord::new(5, 22),
                TetriminoType::L,
            ),
            TetriminoType::O => Tetrimino::spawn_tetrimno(
                vec![
                    Coord::new(0, 0),
                    Coord::new(1, 0),
                    Coord::new(0, 1),
                    Coord::new(1, 1),
                ],
                Coord::new(5, 22),
                TetriminoType::O,
            ),
            TetriminoType::S => Tetrimino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(1, 1),
                    Coord::new(2, 1),
                ],
                Coord::new(5, 22),
                TetriminoType::S,
            ),
            TetriminoType::T => Tetrimino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(1, 1),
                    Coord::new(2, 0),
                ],
                Coord::new(5, 22),
                TetriminoType::T,
            ),
            TetriminoType::Z => Tetrimino::spawn_tetrimno(
                vec![
                    Coord::new(1, 0),
                    Coord::new(2, 0),
                    Coord::new(0, 1),
                    Coord::new(1, 1),
                ],
                Coord::new(5, 22),
                TetriminoType::Z,
            )
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_t_tetrimino_spawn() {
        let tetrimino = Tetrimino::spawn_tetrimno(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            Coord::new(5, 22),
            TetriminoType::T,
        );
        let right_real_coords = vec![
            Coord { x: 4, y: 22 },
            Coord { x: 5, y: 23 },
            Coord { x: 5, y: 22 },
            Coord { x: 6, y: 22 },
        ];
        dbg!(&right_real_coords, tetrimino.coords());

        for idx in (0..4).into_iter() {
            assert_eq!(right_real_coords.get(idx), tetrimino.coords().get(idx))
        }
    }
    #[test]
    fn test_boundary_positive() {
        let tetrimino = Tetrimino::spawn_tetrimno(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            Coord::new(5, 10),
            TetriminoType::T,
        );
        assert!(tetrimino.within_boundary(Direction::Down));
    }

    #[test]
    fn test_boundary_negative() {
        let tetrimino = Tetrimino::spawn_tetrimno(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            Coord::new(5, 0),
            TetriminoType::T,
        );
        assert!(tetrimino.within_boundary(Direction::Down));
    }
}
