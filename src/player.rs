use raylib::prelude::*;

pub struct Player {
    position: Vector3,
    model: Model,
    speed: f32,
    rotation_angle: f32,
}

impl Player {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let model = rl
            .load_model(thread, "assets/Rogue.glb")
            .expect("Failed to load player model");

        Player {
            model,
            position: Vector3::new(0.0, 0.0, 0.0),
            speed: 5.0,
            rotation_angle: 0.0,
        }
    }

    pub fn draw(&mut self, d3d: &mut RaylibMode3D<RaylibDrawHandle>) {
        d3d.draw_model_ex(
            &self.model,
            self.position,
            Vector3::new(0.0, 0.0, 0.0),
            self.rotation_angle,
            Vector3::new(1.0, 1.0, 1.0),
            Color::WHITE,
        );
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, delta_time: f32) {
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.position.z += self.speed * delta_time;
        } else if rl.is_key_down(KeyboardKey::KEY_W) {
            self.position.z -= self.speed * delta_time;
        }
    }
}
