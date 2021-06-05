// Game should hold all our components
use crate::Universe;
use crate::TetriminoControls;

pub struct Game {
    components: Vec<Box<GameComponents>>
}

impl Game {
    
}

impl Default for Game {
    fn default() -> Self {
        // Universe itself just holds data?
        let mut universe = Box::new(Universe::default());
        // Tetrimino controls holds player systems
        let tetrimino_controls = Box::new(TetriminoControls::new());
        // Physics controls holds physics systems (rendering, etc.)
        let components = vec![universe, tetrimino_controls];
        // let physics = 

    }
}

struct GameComponents {
    
}
