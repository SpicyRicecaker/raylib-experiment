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

mod spawn {
    use super::super::*;
    use rand::{distributions::Standard, prelude::Distribution, Rng};

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
                TetrominoType::I => Tetromino::spawn_tetromino(
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(2, 0),
                        Coord::new(3, 0),
                    ],
                    Coord::new(5, 22),
                    TetrominoType::I,
                ),
                TetrominoType::J => Tetromino::spawn_tetromino(
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(2, 0),
                        Coord::new(0, 1),
                    ],
                    Coord::new(5, 22),
                    TetrominoType::J,
                ),
                TetrominoType::L => Tetromino::spawn_tetromino(
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(2, 0),
                        Coord::new(2, 1),
                    ],
                    Coord::new(5, 22),
                    TetrominoType::L,
                ),
                TetrominoType::O => Tetromino::spawn_tetromino(
                    vec![
                        Coord::new(0, 0),
                        Coord::new(1, 0),
                        Coord::new(0, 1),
                        Coord::new(1, 1),
                    ],
                    Coord::new(5, 22),
                    TetrominoType::O,
                ),
                TetrominoType::S => Tetromino::spawn_tetromino(
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(1, 1),
                        Coord::new(2, 1),
                    ],
                    Coord::new(5, 22),
                    TetrominoType::S,
                ),
                TetrominoType::T => Tetromino::spawn_tetromino(
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(1, 1),
                        Coord::new(2, 0),
                    ],
                    Coord::new(5, 22),
                    TetrominoType::T,
                ),
                TetrominoType::Z => Tetromino::spawn_tetromino(
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
}