use raylib::prelude::*;
use raylib_moving_cube::{Entity, Loop};
use raylib_moving_cube::Player;

struct Config<'a> {
    fps: u32,
    w: u32,
    h: u32,
    title: &'a str
}

struct Universe {
    entities: Vec<Entity>,
}

impl Universe {
    fn tick(&mut self, rl: &RaylibHandle) {
        let mut bob = &mut self.entities;

        for e in bob {
            match e {
                Entity::Player(p) => {
                    p.tick(rl)
                }
            }
        }
    }
    
    fn entities () {

    }
    
    // fn entities_mut (&mut self, index: usize) -> &mut Entity {
    //     &mut self
    // }
    
    fn render (&self, d: &mut RaylibDrawHandle) {
        for e in &self.entities {
            match e {
                Entity::Player(p) => {
                    p.render(d);
                }
            }
        }

    }
}

fn main() {
    let config = Config { fps: 60, w: 1920, h: 1080, title: "Game"};
    let mut universe = Universe { entities: vec![] };
    let player = Player { x: config.w/2, y: config.h/2, radius: config.w/50, color: Color::from_hex("d4be98").unwrap()};

    universe.entities.push(Entity::Player(player));

    let (mut rl, thread) = raylib::init().size(config.w as i32, config.h as i32).title(config.title).build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        universe.tick(&rl);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::from_hex("292828").unwrap());
        
        universe.render(&mut d);
        
    }
}
