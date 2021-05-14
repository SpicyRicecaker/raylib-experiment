pub mod Utils {

    #[derive(Clone, Copy)]
    pub enum KeyboardState {
        Initiation,
        Held,
    }
    impl KeyboardState {
        fn default() -> KeyboardState {
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
        /// Get a reference to the controlled key's key.
        pub fn key(&self) -> &raylib::consts::KeyboardKey {
            &self.key
        }

        /// Set the controlled key's buffer.
        pub fn set_buffer(&mut self, buffer: Buffer) {
            self.buffer = buffer;
        }

        pub fn close_buffer(&mut self) {
            self.buffer = Buffer::Closed
        }

        pub fn reset_buffer(&mut self) {
            self.buffer = Buffer::Opened(0)
        }

        /// Get a reference to the controlled key's buffer.
        pub fn buffer(&self) -> &Buffer {
            &self.buffer
        }

        pub fn increment_buffer(&mut self) {
            if let Buffer::Opened(i) = &mut self.buffer {
               *i += 1
            }
        }

        /// Get a reference to the controlled key's state.
        pub fn state(&self) -> &KeyboardState {
            &self.state
        }

        /// Set the controlled key's state.
        pub fn set_state(&mut self, state: KeyboardState) {
            self.state = state;
        }
    
    /// Get a mutable reference to the controlled key's buffer.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}

    // impl ControlledKey {
    //     fn from (key: raylib::consts::KeyboardKey, repeat: ) -> Self {
    //         ControlledKey {
    //             key,
    //             state: KeyboardState::Initiation,
    //             buffer: Buffer::Closed,
    //             repeat: Repeat { delay: },
    //         }
    //     }
    // }

    impl Default for ControlledKey {
        fn default() -> Self {
            // Defaults are, as per usual, scuffed
            ControlledKey {
                key: raylib::consts::KeyboardKey::KEY_A,
                state: KeyboardState::Initiation,
                buffer: Buffer::Closed,
                repeat: Repeat { delay: 8, rate: 4 },
            }
        }
    }
}

pub mod Tetris {
    // Utils for holding a key
    use super::Utils::*;
    // The framework that keyboard input and keys are built on
    use raylib::prelude::*;
    // Our implementation of tetriminos
    use crate::Tetrimino;
    pub struct TetriminoControls {
        // Not sure if fallrate really fits the agenda here
        FALLRATE: u32,
        controlled_keys: Vec<ControlledKey>,
    }

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

        /// Get a reference to the universe's fallrate.
        pub fn FALLRATE(&self) -> &u32 {
            &self.FALLRATE
        }

        pub fn tick(&mut self, rl: &RaylibHandle, tetrimino: &mut Tetrimino) {
            // for ckey in &mut self.controlled_keys.iter() {
            let ckey = self.controlled_keys.get_mut(0).unwrap();

            if rl.is_key_pressed(*ckey.key()) {
                ckey.close_buffer();
                tetrimino.move_right();
            }

            // let state = *ckey.state();
            if let Buffer::Opened(buffer) = ckey.buffer() {
                match ckey.state() {
                    KeyboardState::Initiation => {
                        if rl.is_key_down(*ckey.key()) {
                            ckey.increment_buffer();
                            dbg!(&buffer);
                            // if *buffer > ckey.repeat.delay {
                            //     ckey.set_state(KeyboardState::Held);
                            //     ckey.reset_buffer();
                            // }
                        } else {
                            ckey.close_buffer();
                        }
                    }
                    KeyboardState::Held => {
                        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                            ckey.increment_buffer();
                            if *buffer > ckey.repeat.rate {
                                // move
                                tetrimino.move_right();
                                ckey.reset_buffer()
                            }
                        } else {
                            ckey.set_state(KeyboardState::Initiation);
                            ckey.close_buffer();
                        }
                    }
                }
            }
            // }
        }
    }
}
