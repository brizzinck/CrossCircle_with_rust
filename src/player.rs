use crate::state::PlayerState;

#[derive(Default)]
pub struct Player {
    pub current_state: PlayerState,
}
impl Player {
    pub fn change_state(&mut self) {
        self.current_state = match self.current_state {
            PlayerState::Circle => PlayerState::Cross,
            PlayerState::Cross => PlayerState::Circle
        }
    }
}