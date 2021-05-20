mod tetriminos;
mod tetris_input;

use raylib::prelude::*;
use tetriminos::*;

use tetris_input::tetris::TetriminoControls;
// use raylib::consts::KeyboardKey;
// use raylib::{prelude::RaylibDrawHandle, RaylibHandle};

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
    focused_tetrimino: Tetrimino,
    stagnant_tetriminos: Vec<Tetrimino>,
    ticks: u32,
    tetrimino_controls: TetriminoControls,
}

impl Universe {
    pub fn new(
        w: u32,
        h: u32,
        focused_tetrimino: Tetrimino,
        stagnant_tetriminos: Vec<Tetrimino>,
        ticks: u32,
    ) -> Self {
        let tetrimino_controls = TetriminoControls::new();
        Universe {
            w,
            h,
            focused_tetrimino,
            stagnant_tetriminos,
            ticks,
            tetrimino_controls,
        }
    }

    fn fall_focused(&mut self) {
        // Code that determines moving the pieces down
        let within_boundary = self.focused_tetrimino().within_boundary(Direction::Down);
        let mut collision = false;

        if within_boundary {
            for stagnant_tetrimino in self.stagnant_tetriminos() {
                if Tetrimino::will_collide(
                    self.focused_tetrimino(),
                    stagnant_tetrimino,
                    Direction::Down,
                ) {
                    collision = true;
                }
            }
        }

        if !collision && within_boundary {
            self.focused_tetrimino_mut().move_down();
        } else {
            // Solidify the old current
            let temp = self.focused_tetrimino.clone();
            // let temp = self.focused_tetrimino.clone();
            self.stagnant_tetrimino_mut().push(temp);
            // We need to generate a new current and solidify the old current
            *self.focused_tetrimino_mut() =
                TetriminoType::generate_tetrimino_from_type(TetriminoType::T);
        }
    }

    pub fn tick(&mut self, rl: &RaylibHandle) {
        *self.ticks_mut() += 1;
        let tick_value = *self.ticks();

        // Literally just move current .y down
        self.tetrimino_controls
            .tick(rl, &mut self.focused_tetrimino);
        // Falls at the rate of 6 per second

        if self.ticks() % 12 == 0 {
            self.fall_focused();
        }

        if *self.ticks() >= 60 {
            *self.ticks_mut() = 0;
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
        self.focused_tetrimino().render(d, config);

        // Draw every coord
        for tetrimino in self.stagnant_tetriminos().iter() {
            tetrimino.render(d, config);
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
        Universe::new(
            10,
            40,
            TetriminoType::generate_tetrimino_from_type(TetriminoType::T),
            vec![],
            0,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_down() {
        let mut tetrimino = Tetrimino::generate_tetrimino_from_center(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            Coord::new(1, 0),
            TetriminoType::T,
            Coord::new(5, 22),
        );
        tetrimino.move_down();

        let right_real_coords = vec![
            Coord { x: 4, y: 21 },
            Coord { x: 5, y: 22 },
            Coord { x: 5, y: 21 },
            Coord { x: 6, y: 21 },
        ];

        dbg!(&right_real_coords, tetrimino.real_coords());

        for idx in (0..4).into_iter() {
            assert_eq!(right_real_coords.get(idx), tetrimino.real_coords().get(idx))
        }
    }
    #[test]
    fn test_move_down_3() {
        let mut tetrimino = Tetrimino::generate_tetrimino_from_center(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            Coord::new(1, 0),
            TetriminoType::T,
            Coord::new(5, 22),
        );
        tetrimino.move_down();
        tetrimino.move_down();
        tetrimino.move_down();

        let right_real_coords = vec![
            Coord { x: 4, y: 19 },
            Coord { x: 5, y: 20 },
            Coord { x: 5, y: 19 },
            Coord { x: 6, y: 19 },
        ];

        dbg!(&right_real_coords, tetrimino.real_coords());

        for idx in (0..4).into_iter() {
            assert_eq!(right_real_coords.get(idx), tetrimino.real_coords().get(idx))
        }
    }
}
