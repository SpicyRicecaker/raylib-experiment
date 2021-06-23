use super::rotations::rotation_direction::RotationDirection;
use super::{direction::*, Tetromino};
use super::{InputInterface, Universe};

use raylib::prelude::*;

impl InputInterface for Universe {
    fn receive_key(&mut self) {
        for key in self.tetromino_controls.get_queue() {
            match key {
                KeyboardKey::KEY_LEFT => {
                    let dxdy = Tetromino::get_dxdy(Direction::Left);
                    if self.focused_tetromino.within_boundary(dxdy, &self.dim)
                        && !Tetromino::will_collide_all(
                            &self.focused_tetromino,
                            &self.stagnant_tetrominos,
                            dxdy,
                        )
                    {
                        self.focused_tetromino.move_by(dxdy)
                    }
                }
                KeyboardKey::KEY_RIGHT => {
                    let dxdy = Tetromino::get_dxdy(Direction::Right);
                    if self.focused_tetromino.within_boundary(dxdy, &self.dim)
                        && !Tetromino::will_collide_all(
                            &self.focused_tetromino,
                            &self.stagnant_tetrominos,
                            dxdy,
                        )
                    {
                        self.focused_tetromino.move_by(dxdy)
                    }
                }
                KeyboardKey::KEY_DOWN => {
                    self.fall_focused();
                    self.game.fast_move_down_score()
                }
                KeyboardKey::KEY_Z => self.rotate_focused(RotationDirection::CounterClockwise),
                KeyboardKey::KEY_C => self.rotate_focused(RotationDirection::Clockwise),
                KeyboardKey::KEY_SPACE => {
                    let lines = self.focused_tetromino.coords()[0].y - self.ghost.coords()[0].y;
                    self.focused_tetromino = self.ghost.clone();
                    self.fall_focused();
                    self.game.hard_move_down_score(lines);
                }
                _ => {}
            }
        }
        self.tetromino_controls.clear_queue();
    }
}
