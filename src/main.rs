use crate::player::Player;
use raylib::prelude::*;

mod player;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 600).title("Rogue Engine").build();

    let mut player = Player::new(&mut rl, &thread);

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        player.update(&mut rl, delta_time);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        player.draw(&mut d);
        
    }
}
