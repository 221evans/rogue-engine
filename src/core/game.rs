use crate::core::player::Player;
use crate::core::zombie::Zombie;
use raylib::prelude::*;
use crate::core::combat::Combat;
use crate::core::GameState;

pub struct Game {
    player: Player,
    zombies: Vec<Zombie>,
    game_state: GameState,
    combat: Combat,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {

        let zombies = Vec::new();

        Game {
            player: Player::new(rl, thread, 200.0,200.0),
            zombies,
            game_state: GameState::FreeRoam,
            combat: Combat::new(),
        }
    }

    pub fn init(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.spawn_zombies(rl,thread);
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {

        match self.game_state {
            GameState::FreeRoam => {
                self.player.draw(d);
                for z in &mut self.zombies {
                    z.draw(d);
                }
            }
            GameState::Combat => {

                self.player.draw(d);
                for z in &mut self.zombies {
                    z.draw(d);
                }

                self.combat.handle_combat_drawing(d);
            }
        }

    }

    pub fn update(&mut self, rl: &mut RaylibHandle, delta_time: f32) {


        match self.game_state {
            GameState::FreeRoam => {
                self.player.update(rl, delta_time);
                for z in &mut self.zombies {
                    z.update();

                    if check_for_rectangle_overlap(z.collision_rect, self.player.collision_rect) {
                        println!("Zombies overlap");
                        self.game_state = GameState::Combat;
                    }
                    else {
                        println!("Zombies Not Overlapping");
                    }
                }
            }
            GameState::Combat => {
                self.player.dest_rect.x = 200.0;
                self.player.dest_rect.y = 200.0;
                self.player.collision_rect = self.player.dest_rect;
                for z in &mut self.zombies {
                    z.dest_rect.x = 400.0;
                    z.dest_rect.y = 200.0;
                    z.collision_rect = z.dest_rect;
                }
            }
        }


    }

    pub fn spawn_zombies(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.zombies.push(Zombie::new(rl,thread,400.0,200.0));
    }
}

fn check_for_rectangle_overlap(rec1: Rectangle, rec2: Rectangle) -> bool {
    if rec1.x + rec1.width < rec2.x || rec2.x + rec2.width < rec1.x {
        return false;
    }
    if rec1.y + rec1.height < rec2.y || rec2.y + rec2.height < rec1.y {
        return false;
    }
    true
}