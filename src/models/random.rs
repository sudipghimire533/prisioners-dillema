use crate::game;
use game::decision::Decision;

/// a random decision with 50/50 chance of each binary decision
fn random_decision() -> Decision {
    if rand::random::<u32>() % 2 == 0 {
        Decision::Cooperate
    } else {
        Decision::Sniff
    }
}

/// a random decision with slight inclination to cooperate more
/// 2:1 decision of cooperation to sniffing
fn random_positive_decision() -> Decision {
    if rand::random::<u32>() % 3 == 0 {
        Decision::Cooperate
    } else {
        Decision::Sniff
    }
}

/// a random decision with slight inclination to sniffe more
/// 1:2 decision of cooperation to sniffing
fn random_negative_decision() -> Decision {
    if rand::random::<u32>() % 3 == 0 {
        Decision::Sniff
    } else {
        Decision::Cooperate
    }
}

/// A player that decides what to do on random
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RandomDecider;
impl<P1, P2> game::player::PlayerConcious<game::game::Game::<P1, P2>> for RandomDecider {
    fn opening_move(&self, _game_config: &game::game::Game::<P1, P2>) -> Decision {
        // decide in random
        random_decision()
    }

    fn decide_first_move(&self, _game_context: &game::game::Game::<P1, P2>) -> Decision {
        // decide in random
        random_decision()
    }

    fn decide_second_move(
        &self,
        _game_context: &game::game::Game::<P1, P2>,
        _opponent_decision: &Decision,
    ) -> game::decision::Decision {
        // decide in random
        random_decision()
    }
}

/// A player who decides random but with more inclination to cooperation
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RandomPositiveDecider;
impl<P1, P2> game::player::PlayerConcious<game::game::Game::<P1, P2>> for RandomPositiveDecider {
    fn opening_move(&self, _game_config: &game::game::Game::<P1, P2>) -> Decision {
        // decide in random
        random_positive_decision()
    }

    fn decide_first_move(&self, _game_context: &game::game::Game::<P1, P2>) -> Decision {
        random_positive_decision()
    }

    fn decide_second_move(
        &self,
        _game_context: &game::game::Game::<P1, P2>,
        _opponent_decision: &Decision,
    ) -> Decision {
        random_positive_decision()
    }
}

/// A player who decides random but with more inclination to sniffing
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RandomNegativeDecider;
impl<P1, P2> game::player::PlayerConcious<game::game::Game::<P1, P2>> for RandomNegativeDecider {
    fn opening_move(&self, _game_config: &game::game::Game::<P1, P2>) -> Decision {
        // decide in random
        random_negative_decision()
    }

    fn decide_first_move(&self, _game_context: &game::game::Game::<P1, P2>) -> Decision {
        random_negative_decision()
    }

    fn decide_second_move(
        &self,
        _game_context: &game::game::Game::<P1, P2>,
        _opponent_decision: &Decision,
    ) -> Decision {
        random_negative_decision()
    }
}
