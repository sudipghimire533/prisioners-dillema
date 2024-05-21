use crate::game;
use game::decision::Decision;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AlwaysCooperative {}
impl game::player::PlayerConcious<game::game::Game> for AlwaysCooperative {
    fn opening_move(&self, _game_config: &game::game::Game) -> Decision {
        Decision::Cooperate
    }

    fn decide_first_move(&self, _game_context: &game::game::Game) -> Decision {
        Decision::Cooperate
    }

    fn decide_second_move(
        &self,
        _game_context: &game::game::Game,
        _opponent_decision: &Decision,
    ) -> Decision {
        Decision::Cooperate
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AlwaysDefect;

impl game::player::PlayerConcious<game::game::Game> for AlwaysDefect {
    fn opening_move(&self, _game_config: &game::game::Game) -> Decision {
        Decision::Sniff
    }

    fn decide_first_move(&self, _game_context: &game::game::Game) -> Decision {
        Decision::Sniff
    }

    fn decide_second_move(
        &self,
        _game_context: &game::game::Game,
        _opponent_decision: &Decision,
    ) -> Decision {
        Decision::Sniff
    }
}
