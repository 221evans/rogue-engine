use raylib::prelude::*;

pub struct Zombie {
    idle_side_texture: Texture2D,
    src_rect: Rectangle,
    dest_rect: Rectangle,
    pub collision_rect: Rectangle,
}

impl Zombie {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, pos_x: f32, pos_y: f32) -> Self {

        let idle_side_texture = rl.load_texture(thread, "assets/zombie/Idle-Sheet.png")
            .expect("Failed to load soldier texture");

        Zombie {
            idle_side_texture,
            src_rect: Rectangle::new(0.0,0.0,64.0,64.0),
            dest_rect: Rectangle::new(pos_x,pos_y,64.0,64.0),
            collision_rect: Rectangle::new(0.0,0.0,64.0,64.0),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_texture_pro(
          &self.idle_side_texture,
          self.src_rect,
          self.dest_rect,
          Vector2::new(0.0,0.0),
          0.0,
          Color::WHITE,
        );

        d.draw_rectangle_lines_ex(
            &self.collision_rect,
            1.0,
            Color::RED,
        )
    }
    
    pub fn update(&mut self) {
        self.collision_rect = self.dest_rect;
    }
}