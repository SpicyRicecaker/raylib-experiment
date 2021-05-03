pub struct Tetrimino {
    coords: [[u8; 2]; 4],
    center: [u8; 2],
    real: [u32; 2],
    tetrimino_type: TetriminoType,
}

impl Tetrimino {
    pub fn new(
        coords: [[u8; 2]; 4],
        center: [u8; 2],
        real: [u32; 2],
        tetrimino_type: TetriminoType,
    ) -> Self {
        Tetrimino {
            coords,
            center,
            real,
            tetrimino_type,
        }
    }

    pub fn coords(&self) -> [[u8; 2]; 4] {
        self.coords
    }
    pub fn real(&self) -> [u32; 2] {
        self.real
    }
    pub fn real_mut(&mut self) -> [u32; 2] {
        self.real
    }
    pub fn center(&self) -> [u8; 2] {
        self.center
    }
}

pub enum TetriminoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetriminoType {
    pub fn generate_tetrimino_from_type(tetrimino_type: TetriminoType) -> Tetrimino {
        match tetrimino_type {
            // TetriminoType::I => {}
            // TetriminoType::J => {}
            // TetriminoType::L => {}
            // TetriminoType::O => {}
            // TetriminoType::S => {}
            TetriminoType::T => Tetrimino::new(
                [[0, 1], [1, 0], [1, 1], [2, 1]],
                [1, 1],
                [22, 5],
                TetriminoType::T,
            ),
            // TetriminoType::Z => {}
            _ => Tetrimino::new(
                [[0, 1], [1, 0], [1, 1], [2, 1]],
                [1, 1],
                [22, 5],
                TetriminoType::T,
            ),
        }
    }
}
