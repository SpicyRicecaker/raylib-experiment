use raylib::prelude::*;
use tetris_raylib_rs::{config::Config, universe::Universe};

fn main() {
    let config = Config::default();
    let mut universe = Universe::default();

    init();

    let (mut rl, thread) = raylib::init()
        .size(*config.w() as i32, *config.h() as i32)
        .title(&config.title()[..])
        .build();

    rl.set_target_fps(*config.fps());

    let mut audio = RaylibAudio::init_audio_device();
    // match Music::load_music_stream(&thread, "../resources/cool.wav") {
    //     Ok(mut m) => RaylibAudio::play_music_stream(&mut audio, &mut m),
    //     Err(e) => {
    //         dbg!(e, "something happened with audio");
    //     }
    // }

    // Debug, create new tetromino and add it to the universe
    while !rl.window_should_close() {
        universe.tick(&rl);

        let mut d = rl.begin_drawing(&thread);

        universe.render(&mut d, &config);
    }
}
