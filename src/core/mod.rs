pub mod game;
mod player;
mod zombie;
mod combat;


enum GameState {
    FreeRoam,
    Combat,
}

enum FaceDirection {
    Side,
    Up,
    Down,
}

enum CombatTurn {
    Player,
    Enemy,
}
