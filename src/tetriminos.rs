pub struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Self {
        // Initialized as x, y
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

pub struct Tetrimino {
    coords: Vec<Coord>,
    center: Coord,
    real: Coord,
    tetrimino_type: TetriminoType,
}

impl Tetrimino {
    pub fn new(
        coords: Vec<Coord>,
        center: Coord,
        real: Coord,
        tetrimino_type: TetriminoType,
    ) -> Self {
        Tetrimino {
            coords,
            center,
            real,
            tetrimino_type,
        }
    }

    pub fn coords(&self) -> &Vec<Coord> {
        &self.coords
    }
    pub fn real(&self) -> &Coord {
        &self.real
    }
    pub fn center(&self) -> &Coord {
        &self.center
    }

    /// Get a mutable reference to the tetrimino's real.
    pub fn real_mut(&mut self) -> &mut Coord {
        &mut self.real
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
                vec![
                    Coord::new(0, 1),
                    Coord::new(1, 0),
                    Coord::new(1, 1),
                    Coord::new(2, 1),
                ],
                Coord::new(1, 1),
                Coord::new(5, 22),
                TetriminoType::T,
            ),
            // TetriminoType::Z => {}
            _ => Tetrimino::new(
                vec![
                    Coord::new(0, 1),
                    Coord::new(1, 0),
                    Coord::new(1, 1),
                    Coord::new(2, 1),
                ],
                Coord::new(1, 1),
                Coord::new(5, 22),
                TetriminoType::T,
            ),
        }
    }
}
