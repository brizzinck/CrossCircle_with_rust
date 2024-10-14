#[derive(Default)]
pub enum PlayerState {
    #[default]
    Cross = 0,
    Circle = 1,
}
#[derive(Default)]
pub enum GameState {
    #[default]
    Bootstrap = 0,
    Game = 1,
}