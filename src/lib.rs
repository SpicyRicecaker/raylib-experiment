// use raylib::consts::KeyboardKey;
// use raylib::{prelude::RaylibDrawHandle, RaylibHandle};
use raylib::prelude::*;
mod tetriminos;
mod Tetriminos;
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
        Self::new(2, 1600, 900, String::from("Tetris"))
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
    focused_tetrimino: Tetrimino,
    stagnant_tetriminos: Vec<Tetrimino>,
}

impl Universe {
    pub fn new(
        w: u32,
        h: u32,
        focused_tetrimino: Tetrimino,
        stagnant_tetriminos: Vec<Tetrimino>,
    ) -> Self {
        Universe {
            w,
            h,
            focused_tetrimino,
            stagnant_tetriminos,
        }
    }

    pub fn tick(&mut self, rl: &RaylibHandle) {
        // Literally just move current .y down
        // let y = self.current_mut().real_mut()[0];
        if *self.focused_tetrimino().real().y() - 1 == 0 {
            // Solidify the old current
            let temp = self.focused_tetrimino.clone();
            // let temp = self.focused_tetrimino.clone();
            self.stagnant_tetrimino_mut().push(temp);
            // We need to generate a new current and solidify the old current
            *self.focused_tetrimino_mut() = TetriminoType::generate_tetrimino_from_type(TetriminoType::T);
        } else {
            *self.focused_tetrimino_mut().real_mut().mut_y() -= 1;
        }
    }

    /// Renders the 10x20 grid that tetriminos spawn on oo
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
        // Render our current tetrimino

        // Find the real world diff between each of the coords
        // let coords_dy = self.current().real().y() - self.current().center().y();
        // let coords_dx = self.current().real().x() - self.current().center().x();

        // Find the size of each cube
        let dy = config.h() / 20;
        let dx = *config.actual_w() as u32 / 10;

        // For every coord in the tetrimino (4 coords in total)
        for coords in self.focused_tetrimino().coords().iter() {
            // First its 'real' coords
            let real_y = self.focused_tetrimino().real().y() + coords.y() - self.focused_tetrimino().center().y();
            // If it's offscreen just return
            if real_y >= 20 {
                continue;
            }
            let real_x = self.focused_tetrimino().real().x() + coords.x() - self.focused_tetrimino().center().x();
            // Figure out what this means in terms of real coords
            d.draw_rectangle(
                (*config.canvas_l() as u32 + real_x * dx) as i32,
                (*config.h() - (real_y + 1) * dy) as i32,
                dx as i32,
                dy as i32,
                if coords.x() == self.focused_tetrimino().center().x()
                    && coords.y() == self.focused_tetrimino().center().y()
                {
                    Color::from_hex("d4be98").unwrap()
                } else {
                    dbg!(real_y);
                    Color::from_hex("ea6962").unwrap()
                },
            )
        }
        
        // Draw every coord
        for tetrimino in self.stagnant_tetriminos().iter() {
            tetrimino.coords()
        }

        // Render grid
        self.render_grid(d, config);
    }

    /// Get a reference to the universe's current.
    pub fn focused_tetrimino(&self) -> &Tetrimino {
        &self.focused_tetrimino
    }

    /// Get a mutable reference to the universe's current.
    pub fn focused_tetrimino_mut(&mut self) -> &mut Tetrimino {
        &mut self.focused_tetrimino
    }
    
    /// Get a reference to the universe's stagnant tetriminos.
    pub fn stagnant_tetriminos(&self) -> &Vec<Tetrimino> {
        &self.stagnant_tetriminos
    }

    pub fn stagnant_tetrimino_mut(&mut self) -> &mut Vec<Tetrimino> {
        &mut self.stagnant_tetriminos
    }
}

impl Default for Universe {
    fn default() -> Self {
        Universe::new(
            10,
            40,
            TetriminoType::generate_tetrimino_from_type(TetriminoType::T),
            vec![]
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
