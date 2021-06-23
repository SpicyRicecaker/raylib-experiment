// Utils for holding a key
mod utils;

use utils::*;
// The framework that keyboard input and keys are built on
use raylib::prelude::*;

pub trait InputInterface {
    fn receive_key(&mut self);
}
// Our implementation of tetrominos
pub struct TetrominoControls {
    // Not sure if fallrate really fits the agenda here
    controlled_keys: Vec<ControlledKey>,
    queue: Vec<KeyboardKey>,
}

// This implementation isn't gonna work, if we have for example more functions that we want the keys to do than move the tetromino
impl TetrominoControls {
    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }

    pub fn get_queue(&self) -> Vec<KeyboardKey> {
        self.queue.clone()
    }

    pub fn tick(&mut self, rl: &RaylibHandle) {
        for controlled_key in self.controlled_keys.iter_mut() {
            if controlled_key.tick(rl) {
                self.queue.push(controlled_key.key)
            }
        }
    }
}

impl Default for TetrominoControls {
    fn default() -> Self {
        let controlled_keys = vec![
            ControlledKey {
                key: KeyboardKey::KEY_LEFT,
                repeat: Repeat { delay: 8, rate: 4 },
                ..Default::default()
            },
            ControlledKey {
                key: KeyboardKey::KEY_RIGHT,
                repeat: Repeat { delay: 8, rate: 4 },
                ..Default::default()
            },
            ControlledKey {
                key: KeyboardKey::KEY_DOWN,
                repeat: Repeat { delay: 0, rate: 4 },
                ..Default::default()
            },
            ControlledKey {
                key: KeyboardKey::KEY_Z,
                repeat: Repeat { delay: 8, rate: 8 },
                ..Default::default()
            },
            ControlledKey {
                key: KeyboardKey::KEY_C,
                repeat: Repeat { delay: 8, rate: 8 },
                ..Default::default()
            },
            ControlledKey {
                key: KeyboardKey::KEY_SPACE,
                repeat: Repeat { delay: 8, rate: 8 },
                ..Default::default()
            },
        ];
        TetrominoControls {
            controlled_keys,
            queue: Vec::new(),
        }
    }
}
