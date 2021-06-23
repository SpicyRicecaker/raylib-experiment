pub mod color;
mod input;
mod rotations;

use std::collections::HashSet;

use rotations::rotation_direction::RotationDirection;
use tetromino::tetromino_type::TetrominoType;

use color::ColorPalette;

use super::*;
use direction::*;

const INITIAL_WIDTH: u32 = 10;
const INITIAL_HEIGHT: u32 = 20;
const INITIAL_SPEED: u32 = 12;

// Single, double, triple, tetris, based off of gameboy
const SCORE: [u32; 4] = [40, 100, 300, 1200];
// Speeds for levels 3-20, based off of gameboy
const SPEEDS: [u32; 21] = [
    53, 49, 45, 41, 37, 33, 28, 22, 17, 11, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 3,
];
const LVL_CAP: u32 = 20;

pub struct Game {
    // Internal game tick
    ticks: u32,
    // Game running
    running: bool,
    // Score
    lines_cleared: u32,
    // level
    level: u32
}
impl Game {
    /// Get a reference to the game's running.
    pub fn running(&self) -> &bool {
        &self.running
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

impl Game {
    pub fn pause(&mut self) {
        self.running = false;
    }
    pub fn resume(&mut self) {
        self.running = true;
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            ticks: 0,
            running: true,
            lines_cleared: 0,
            level: 0
        }
    }
}

pub struct Universe {
    // Player controlled tetrimino
    focused_tetromino: Tetromino,
    // Tetriminos on board
    stagnant_tetrominos: Vec<Tetromino>,
    // Controls for tetrimino
    tetromino_controls: TetrominoControls,
    // Board
    w: u32,
    h: u32,
    // Static color palette for game
    color_palette: ColorPalette,
    // Game mechanics
    game: Game,
}

impl Universe {
    pub fn new(
        w: u32,
        h: u32,
        focused_tetromino: Tetromino,
        stagnant_tetrominos: Vec<Tetromino>,
        tetromino_controls: TetrominoControls,
        color_palette: ColorPalette,
        game_mechanics: Game,
    ) -> Self {
        Universe {
            focused_tetromino,
            stagnant_tetrominos,
            tetromino_controls,
            w,
            h,
            color_palette,
            game: game_mechanics,
        }
    }

    fn fall_focused(&mut self) {
        // Code that determines moving the pieces down
        let within_boundary = self.focused_tetromino.within_boundary(
            Tetromino::get_dxdy(Direction::Down),
            self.w,
            self.h,
        );
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
                self.game.pause();
            }
        }
    }

    fn clear(&mut self) {
        self.stagnant_tetrominos.clear();
    }

    fn game_over(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_R) {
            self.clear();
            self.game.resume();
        }
    }

    pub fn tick(&mut self, rl: &RaylibHandle) {
        if !self.game.running() {
            self.game_over(rl);
            return;
        }

        // Set level of the game

        self.game.ticks += 1;

        self.tetromino_controls.tick(rl);
        self.receive_key();

        // Literally just move current .y down
        // Falls at the rate of 6 per second

        if self.game.ticks % 12 == 0 {
            self.fall_focused();
        }

        if self.game.ticks >= 60 {
            self.game.ticks = 0;
        }

        let mut levels: HashMap<u32, u32> = HashMap::new();

        // Setup hash
        // We should probably store the hashmap, this way we won't have to update it every tick
        for tetromino in self.stagnant_tetrominos.iter() {
            for coord in tetromino.coords() {
                // Store the number of tetris parts in each y level
                let e = levels.entry(coord.y).or_insert(0);
                *e += 1;
            }
        }

        // filter out hash for levels that we need
        let levels = levels
            .iter()
            .filter_map(|l| if *l.1 == self.w { Some(*l.0) } else { None })
            .collect::<HashSet<u32>>();

        // Nothing to do if there aren't any full rows
        if levels.is_empty() {
            return;
        }

        // ...Otherwise, if there is a full row...

        // Delete all stagnant tetriminos at these specific y levels
        let mut i = 0;
        while i != self.stagnant_tetrominos.len() {
            let mut j = 0;
            while j != self.stagnant_tetrominos[i].coords().len() {
                if levels.contains(&self.stagnant_tetrominos[i].coords()[j].y) {
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

        // Then prepare to move the other tetriminos down (gravity)
        let mut diff = vec![0; self.h as usize];
        for level in levels.iter() {
            Universe::change_arr_from_idx(&mut diff, *level, 1);
        }

        // Finally,if something happened try to move pieces down if they need to be moved
        // fk, we're iterating over stagnant tetrominos like 3 times. We honestly only need to really do it twice if we store the hashmap
        // If we implemented it with an array we would only need to iterate over the board once
        for i in 0..self.stagnant_tetrominos.len() {
            for j in 0..self.stagnant_tetrominos[i].coords().len() {
                self.stagnant_tetrominos[i].coords_mut()[j].y -=
                    diff[self.stagnant_tetrominos[i].coords()[j].y as usize];
            }
        }

        // Add lines cleared to game
        self.game.lines_cleared += levels.len() as u32;
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

        let dx = *config.actual_w() as u32 / self.w;
        // let dy = config.h() / self.h;

        for x in [0, self.w].iter() {
            let current_x = x * dx + *config.canvas_l() as u32;
            d.draw_line_ex(
                Vector2 {
                    x: current_x as f32,
                    y: 0_f32,
                },
                Vector2 {
                    x: current_x as f32,
                    y: *config.h() as f32,
                },
                4_f32,
                self.color_palette.line(),
            );
        }

        // for x in (0..=self.w).into_iter() {
        //     // For every implement of x, draw from the ground to the ceiling
        //     let current_x = x * dx + *config.canvas_l() as u32;
        //     d.draw_line_ex(
        //         Vector2 {
        //             x: current_x as f32,
        //             y: 0_f32,
        //         },
        //         Vector2 {
        //             x: current_x as f32,
        //             y: *config.h() as f32,
        //         },
        //         0.5_f32,
        //         self.color_palette.line(),
        //     );
        // }
        // for y in (0..=self.h).into_iter() {
        //     let current_y = y * dy;
        //     d.draw_line_ex(
        //         Vector2 {
        //             x: *config.canvas_l() as f32,
        //             y: current_y as f32,
        //         },
        //         Vector2 {
        //             x: *config.canvas_r() as f32,
        //             y: current_y as f32,
        //         },
        //         0.5_f32,
        //         self.color_palette.line(),
        //     );
        // }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, config: &Config) {
        // Clear background
        d.clear_background(self.color_palette.grid());

        // Render grid
        self.render_grid(d, config);

        // Render the focused tetrimino
        self.focused_tetromino()
            .render(d, config, self.w, self.h, &self.color_palette);

        // And every other tetrimino
        for tetromino in self.stagnant_tetrominos().iter() {
            tetromino.render(d, config, self.w, self.h, &self.color_palette);
        }

        // If game is in an 'over' state
        if !self.game.running() {
            d.draw_text(
                "GAME",
                150,
                (*config.h() as f64 / 2_f64) as i32,
                100,
                self.color_palette.line(),
            );
            d.draw_text(
                "OVER",
                (*config.w() - 400) as i32,
                (*config.h() as f64 / 2_f64) as i32,
                100,
                self.color_palette.line(),
            );
            d.draw_text(
                "Press \"r\" to restart",
                150,
                (*config.h() as f64 / 2_f64) as i32 + 100,
                20,
                self.color_palette.line(),
            );
        }
    }
}

// Getters and setters
impl Universe {
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
}

impl Default for Universe {
    fn default() -> Self {
        Universe::new(
            INITIAL_WIDTH,
            INITIAL_HEIGHT,
            TetrominoType::generate_tetromino_rand(),
            vec![],
            TetrominoControls::default(),
            ColorPalette::default(),
            Game::default(),
        )
    }
}
