use raylib::prelude::*;
use crate::core::FaceDirection;

pub struct Player {
    idle_side_texture: Texture2D,
    idle_up_texture: Texture2D,
    idle_down_texture: Texture2D,
    src_rect: Rectangle,
    pub dest_rect: Rectangle,
    pub collision_rect: Rectangle,
    speed: f32,
    is_flipped: bool,
    direction: FaceDirection,
}

impl Player {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, pos_x: f32, pos_y: f32) -> Self {

        let idle_side_texture = rl.load_texture(thread, "assets/player/Idle_base/Idle_Side-Sheet.png")
            .expect("Failed to load player idle side texture");

        let idle_up_texture = rl.load_texture(thread, "assets/player/Idle_base/Idle_UP-Sheet.png")
            .expect("Failed to load player idle up side texture");

        let idle_down_texture = rl.load_texture(thread, "assets/player/Idle_base/Idle_Down-Sheet.png")
            .expect("Failed to load player idle down side texture");

        Player {
            idle_side_texture,
            idle_up_texture,
            idle_down_texture,
            src_rect: Rectangle::new(0.0,0.0,64.0,64.0),
            dest_rect: Rectangle::new(pos_x,pos_y,64.0,64.0),
            collision_rect: Rectangle::new(0.0,0.0,64.0,64.0),
            speed: 100.0,
            is_flipped: false,
            direction: FaceDirection::Side,
            
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {

        let current_texture;

        match self.direction {
            FaceDirection::Side => {
                current_texture = &self.idle_side_texture;
            }
            FaceDirection::Up => {
                current_texture = &self.idle_up_texture;
            }
            FaceDirection::Down => {
                current_texture = &self.idle_down_texture;
            }
        }

        d.draw_texture_pro(
            current_texture,
            self.src_rect,
            self.dest_rect,
            Vector2::new(0.0,0.0),
            0.0,
            Color::WHITE,
        );

        d.draw_rectangle_lines_ex(
            self.collision_rect,
            1.0,
            Color::RED);
    }


    pub fn update(&mut self, rl: &mut RaylibHandle, delta_time: f32) {

        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.dest_rect.y -= self.speed * delta_time;
            self.is_flipped = false;
            self.direction = FaceDirection::Up;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.dest_rect.y += self.speed * delta_time;
            self.is_flipped = false;
            self.direction = FaceDirection::Down;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.dest_rect.x += self.speed * delta_time;
            self.is_flipped = false;
            self.direction = FaceDirection::Side;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.dest_rect.x -= self.speed * delta_time;
            self.is_flipped = true;
            self.direction = FaceDirection::Side;
        }

        self.handle_flip();

        self.collision_rect = self.dest_rect;
    }

    fn handle_flip(&mut self,) {
        if self.is_flipped {
            self.src_rect.width = -64.0;
        }
        else {
            self.src_rect.width = 64.0;
        }
    }
} 

