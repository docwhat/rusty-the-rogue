#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
}

impl TurnState {
    pub fn next(&mut self) {
        *self = match self {
            TurnState::AwaitingInput => TurnState::PlayerTurn,
            TurnState::PlayerTurn => TurnState::EnemyTurn,
            TurnState::EnemyTurn => TurnState::AwaitingInput,
        };
    }
}
