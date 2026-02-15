
use raylib::prelude::*;
use crate::core::game::Game;

mod core;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 600).title("Rogue Engine").build();

    let mut game = Game::new(&mut rl,&thread);
    
    game.init(&mut rl, &thread);

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        game.update(&mut rl, delta_time);
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d);
        d.clear_background(Color::WHITE);
        
    }
}
