use crate::core::player::Player;
use crate::core::zombie::Zombie;
use raylib::prelude::*;
pub struct Game {
    player: Player,
    zombies: Vec<Zombie>,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {

        let zombies = Vec::new();

        Game {
            player: Player::new(rl, thread),
            zombies,
        }
    }

    pub fn init(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.spawn_zombies(rl,thread);
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.player.draw(d);

        for z in &mut self.zombies {
            z.draw(d);
        }

    }

    pub fn update(&mut self, rl: &mut RaylibHandle, delta_time: f32) {
        self.player.update(rl, delta_time);
        for z in &mut self.zombies {
            z.update();
        }
    }

    pub fn spawn_zombies(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.zombies.push(Zombie::new(rl,thread,400.0,200.0));
    }
}