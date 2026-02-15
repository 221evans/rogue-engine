pub mod game;
mod player;
mod zombie;

enum GameState {
    FreeRoam,
    Combat,
}

enum FaceDirection {
    Side,
    Up,
    Down,
}
