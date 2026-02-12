use raylib::prelude::*;

pub struct Player {
    idle_side_texture: Texture2D,
    src_rect: Rectangle,
    dest_rect: Rectangle,
    position: Vector2,
}

impl Player {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {

        let idle_side_texture = rl.load_texture(thread, "assets/player/Idle_base/Idle_Side-Sheet.png")
        .expect("Failed to load player idle side texture");

        Player {
            idle_side_texture,
            src_rect: Rectangle::new(0.0,0.0,64.0,64.0),
            dest_rect: Rectangle::new(0.0,0.0,64.0,64.0),
            position: Vector2::new(0.0,0.0),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_texture_pro(
            &self.idle_side_texture,
            self.src_rect,
            self.dest_rect, Vector2::new(self.position.x,self.position.y),
            0.0,
            Color::WHITE,
        );
    }


    pub fn update(&mut self, rl: &mut RaylibHandle, delta_time: f32) {

        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.dest_rect.y -= 50.0 * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.dest_rect.y += 50.0 * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.dest_rect.x += 50.0 * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.dest_rect.x -= 50.0 * delta_time;
        }
    }
} 

