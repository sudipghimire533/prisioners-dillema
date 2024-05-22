use crate::game;
use game::decision::Decision;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AlwaysCooperative {}
impl<P1, P2> game::player::PlayerConcious<game::game::Game<P1, P2>> for AlwaysCooperative {
    fn opening_move(&self, _game_config: &game::game::Game<P1, P2>) -> Decision {
        Decision::Cooperate
    }

    fn decide_first_move(&self, _game_context: &game::game::Game<P1, P2>) -> Decision {
        Decision::Cooperate
    }

    fn decide_second_move(
        &self,
        _game_context: &game::game::Game<P1, P2>,
        _opponent_decision: &Decision,
    ) -> Decision {
        Decision::Cooperate
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AlwaysDefect;

impl<P1, P2> game::player::PlayerConcious<game::game::Game<P1, P2>> for AlwaysDefect {
    fn opening_move(&self, _game_config: &game::game::Game<P1, P2>) -> Decision {
        Decision::Sniff
    }

    fn decide_first_move(&self, _game_context: &game::game::Game<P1, P2>) -> Decision {
        Decision::Sniff
    }

    fn decide_second_move(
        &self,
        _game_context: &game::game::Game<P1, P2>,
        _opponent_decision: &Decision,
    ) -> Decision {
        Decision::Sniff
    }
}
