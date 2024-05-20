use super::decision::Decision;
use super::game::GameConfig;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlayerId {
    PlayerOne,
    PlayerTwo,
}

pub trait PlayerConcious<GameContext> {
    // opening move i.e first move of 0th round
    fn opening_move(&self, game_config: &GameContext) -> Decision;

    // play as the first move
    fn decide_first_move(&self, game_context: &GameContext) -> Decision;

    // play as the second move
    fn decide_second_move(
        &self,
        game_context: &GameContext,
        opponent_decision: &Decision,
    ) -> Decision;
}
