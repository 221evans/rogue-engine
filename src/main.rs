use crate::player::Player;
use raylib::prelude::*;

mod player;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 600).title("Rogue Engine").build();

    // Create camera
    let camera = Camera3D::orthographic(
        Vector3::new(0.0, 10.0, 10.0), // position
        Vector3::new(0.0, 0.0, 0.0),   // target
        Vector3::new(0.0, 1.0, 0.0),   // up
        20.0,                          // fovy
    );

    let mut player = Player::new(&mut rl, &thread);

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        player.update(&mut rl, delta_time);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        {
            let mut d3d = d.begin_mode3D(camera);
            player.draw(&mut d3d);
        }
    }
}
