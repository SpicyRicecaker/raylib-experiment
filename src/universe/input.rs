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
                    if self.focused_tetromino.within_boundary(dxdy, self.w, self.h)
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
                    if self.focused_tetromino.within_boundary(dxdy, self.w, self.h)
                        && !Tetromino::will_collide_all(
                            &self.focused_tetromino,
                            &self.stagnant_tetrominos,
                            dxdy,
                        )
                    {
                        self.focused_tetromino.move_by(dxdy)
                    }
                }
                KeyboardKey::KEY_DOWN => self.fall_focused(),
                KeyboardKey::KEY_Z => self.rotate_focused(RotationDirection::CounterClockwise),
                KeyboardKey::KEY_C => self.rotate_focused(RotationDirection::Clockwise),
                _ => {}
            }
        }
        self.tetromino_controls.clear_queue();
    }
}
