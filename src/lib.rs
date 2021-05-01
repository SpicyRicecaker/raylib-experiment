// use raylib::consts::KeyboardKey;
// use raylib::{prelude::RaylibDrawHandle, RaylibHandle};
use raylib::prelude::*;

pub trait Loop {
    fn tick(&mut self, rl: &RaylibHandle);
    fn render(&self, d: &mut RaylibDrawHandle);
}

pub struct Player {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
    pub color: Color,
}

impl Player {
    pub fn new(x: u32, y: u32, radius: u32, color: Color) -> Self {
        Player {
            x,
            y,
            radius,
            color,
        }
    }
}

impl Loop for Player {
    fn tick(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.x -= 5;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.x += 5;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.y += 5;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.y -= 5;
        }
    }

    fn render(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, self.radius as f32, self.color)
    }
}

pub enum Entity {
    Player(Player)
}