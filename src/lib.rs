// use raylib::consts::KeyboardKey;
// use raylib::{prelude::RaylibDrawHandle, RaylibHandle};
use raylib::prelude::*;
mod tetriminos;
use tetriminos::*;

pub trait Loop {
    fn tick(&mut self, rl: &RaylibHandle);
    fn render(&self, d: &mut RaylibDrawHandle);
}

pub struct Player {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
    pub color: Color,
    pub velocity: u32,
}

impl Player {
    pub fn new(x: u32, y: u32, radius: u32, color: Color, velocity: u32) -> Self {
        Player {
            x,
            y,
            radius,
            color,
            velocity,
        }
    }
}

impl Loop for Player {
    fn tick(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.x -= self.velocity;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.x += self.velocity;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.y += self.velocity;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.y -= self.velocity;
        }
    }

    fn render(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, self.radius as f32, self.color)
    }
}

pub enum Entity {
    Player(Player),
}

pub struct Config {
    fps: u32,
    w: u32,
    h: u32,
    title: String,
}

impl Config {
    pub fn new(fps: u32, w: u32, h: u32, title: String) -> Self {
        Config { fps, w, h, title }
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
}

impl Default for Config {
    fn default() -> Self {
        Self::new(2, 1920, 1080, String::from("Tetris"))
    }
}

#[derive(Copy, Clone)]
pub enum Cell {
    Occupied = 1,
    Unoccupied = 0,
}

// The board for the tetris board
pub struct Universe {
    cells: [[Cell; 10]; 40],
    w: u32,
    h: u32,
    current: Tetrimino,
}

impl Universe {
    pub fn new(cells: [[Cell; 10]; 40], w: u32, h: u32, current: Tetrimino) -> Self {
        Universe {
            cells,
            w,
            h,
            current,
        }
    }

    pub fn tick(&mut self, rl: &RaylibHandle) {
        // Literally just move current .y down
        // let y = self.current_mut().real_mut()[0];
        self.current_mut().real_mut()[0] -= 1;
    }

    /// Renders the 10x20 grid that tetriminos spawn on oo
    fn render_grid(&self, d: &mut RaylibDrawHandle, config: &Config) {
        // We're gonna give ourselves a margin
        let actual_width = *config.w() as f64 * (9_f64 / 32_f64);
        let left_margin = (*config.w() as f64 - actual_width) / 2_f64;
        let right_margin = left_margin + actual_width;

        // Spawn tetrminoes at up to level 22
        // Only show 10x20 grid

        let dx = actual_width as u32 / 10;
        let dy = config.h() / 20;

        for x in (0..=10).into_iter() {
            // For every implement of x, draw from the ground to the ceiling
            let current_x = x * dx + left_margin as u32;
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
                left_margin as i32,
                current_y as i32,
                right_margin as i32,
                current_y as i32,
                Color::from_hex("d4be98").unwrap(),
            );
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, config: &Config) {
        // Render our current tetrimino
        let dy = self.current().real()[0] - self.current().center()[0] as u32;
        let dx = self.current().real()[1] - self.current().center()[1] as u32;

        for coords in self.current().coords().iter() {
            let actual_width = *config.w() as f64 * (9_f64 / 32_f64);
            let left_margin = (*config.w() as f64 - actual_width) / 2_f64;

        }

        // Render grid
        self.render_grid(d, config);
    }

    /// Get a reference to the universe's current.
    pub fn current(&self) -> &Tetrimino {
        &self.current
    }

    /// Get a mutable reference to the universe's current.
    pub fn current_mut(&mut self) -> &mut Tetrimino {
        &mut self.current
    }
}

impl Default for Universe {
    fn default() -> Self {
        Universe::new(
            [[Cell::Unoccupied; 10]; 40],
            10,
            40,
            TetriminoType::generate_tetrimino_from_type(TetriminoType::T),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tetriminos::TetriminoType;
    #[test]
    fn test_tetrmino_types() {
        let tetrimino = TetriminoType::I;
    }
}
