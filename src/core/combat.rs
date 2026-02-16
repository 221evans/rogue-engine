use crate::core::player::Player;
use crate::core::zombie::Zombie;
use crate::core::CombatTurn;
use raylib::prelude::*;

pub struct Combat {
    pub is_player_turn: bool,
    combat_turn: CombatTurn
}

impl Combat {
    pub fn new() -> Self {

        Combat {
            is_player_turn : true,
            combat_turn: CombatTurn::Player,
        }
    }

    pub fn handle_combat_update(&mut self, player: &mut Player, zombie: &mut Zombie) {
        match self.combat_turn {
            CombatTurn::Player => {

            }
            CombatTurn::Enemy => {

            }
        }
    }

    pub fn handle_combat_drawing(&mut self, d: &mut RaylibDrawHandle) {
        match self.combat_turn {
            CombatTurn::Player => {
                d.draw_text("1: Attack, 2: Defend", 200, 300,10,Color::RED);
            }
            CombatTurn::Enemy => {

            }
        }
    }
}