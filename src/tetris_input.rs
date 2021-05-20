// Should be some sort of component architecture
// Wonder if we should make a tetrimino util, call that from here
pub mod utils {
    use raylib::prelude::*;

    #[derive(Clone, Copy)]
    pub enum KeyboardState {
        Initiation,
        Held,
    }

    impl Default for KeyboardState {
        fn default() -> Self {
            Self::Initiation
        }
    }

    pub enum Buffer {
        Opened(u32),
        Closed,
    }

    pub struct Repeat {
        pub delay: u32,
        pub rate: u32,
    }

    pub struct ControlledKey {
        pub key: raylib::consts::KeyboardKey,
        pub state: KeyboardState,
        pub buffer: Buffer,
        pub repeat: Repeat,
    }

    impl ControlledKey {
        pub fn close_buffer(&mut self) {
            self.buffer = Buffer::Closed
        }

        pub fn open_buffer(&mut self) {
            self.buffer = Buffer::Opened(0)
        }

        pub fn increment_buffer(&mut self) {
            if let Buffer::Opened(i) = &mut self.buffer {
                *i += 1
            }
        }

        /// Set the controlled key's state.
        pub fn set_state(&mut self, state: KeyboardState) {
            self.state = state;
        }
    }

    impl Default for ControlledKey {
        fn default() -> Self {
            // Defaults are, as per usual, scuffed
            ControlledKey {
                key: raylib::consts::KeyboardKey::KEY_A,
                state: KeyboardState::default(),
                buffer: Buffer::Closed,
                repeat: Repeat { delay: 8, rate: 4 },
            }
        }
    }

    impl ControlledKey {
        pub fn tick(&mut self, rl: &RaylibHandle) -> bool {
            if rl.is_key_pressed(self.key) {
                // Reset buffer and move it right
                self.open_buffer();
                return true;
            }

            // Read our current controlled key buffer
            if let Buffer::Opened(buffer) = self.buffer {
                match self.state {
                    // If we're in the first stage after presseed
                    KeyboardState::Initiation => {
                        // If the key is being held
                        if rl.is_key_down(self.key) {
                            // Increment the buffer
                            self.increment_buffer();
                            if buffer > self.repeat.delay {
                                self.state = KeyboardState::Held;
                            }
                            return false;
                        } else {
                            // Otherwise close the buffer
                            self.close_buffer();
                            return false;
                        }
                    }
                    // If the key has been held for a suffcient amount of time
                    KeyboardState::Held => {
                        // If the key is held
                        if rl.is_key_down(self.key) {
                            // Calculate the amount of time that we've held the key
                            self.increment_buffer();
                            // If the # of key presses surpases repeat rate
                            if buffer > self.repeat.rate {
                                // Move the tetrimino, reset buffer count
                                self.open_buffer();
                                return true;
                            }
                            return false;
                        } else {
                            // Otherwise close the buffer
                            self.set_state(KeyboardState::Initiation);
                            self.close_buffer();
                            return false;
                        }
                    }
                }
            }
            false
        }
    }
}

pub mod tetris {
    // Utils for holding a key
    use super::utils::*;
    // The framework that keyboard input and keys are built on
    use raylib::prelude::*;
    // Our implementation of tetriminos
    use crate::Tetrimino;
    pub struct TetriminoControls {
        // Not sure if fallrate really fits the agenda here
        FALLRATE: u32,
        controlled_keys: Vec<ControlledKey>,
    }

    // This implementation isn't gonna work, if we have for example more functions that we want the keys to do than move the tetrimino
    impl TetriminoControls {
        pub fn new() -> Self {
            const FALLRATE: u32 = 6;
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
            ];
            TetriminoControls {
                FALLRATE,
                controlled_keys,
            }
        }

        pub fn tick(&mut self, rl: &RaylibHandle, tetrimino: &mut Tetrimino) {
            for ckey in self.controlled_keys.iter_mut() {
                if ckey.tick(rl) {
                    match ckey.key {
                        KeyboardKey::KEY_LEFT => tetrimino.move_left(),
                        KeyboardKey::KEY_RIGHT => tetrimino.move_right(),
                        _ => {}
                    }
                };
            }
        }
    }
}
