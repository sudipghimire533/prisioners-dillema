use crate::game;
use game::decision::Decision;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TitForTat;
impl game::player::PlayerConcious<game::game::Game> for TitForTat {
    fn opening_move(&self, _game_config: &game::game::Game) -> Decision {
        // ALways start by cooperating
        Decision::Cooperate
    }

    fn decide_first_move(&self, game_context: &game::game::Game) -> Decision {
        // check the last move in last round
        let opponent_last_move = game_context.decision_history.back().map(|round_outcome| {
            // we are inside first move context this means we are the first mover in each round
            // i.e we are player_one
            // so to get opponent's move, this will be player_two
            &round_outcome.player_two_move
        });

        match opponent_last_move {
            // just replicate the last move of opponent
            Some(opponent_last_decision) => opponent_last_decision.clone(),

            // cannot get last move of opponent? we are game opener? .opening_move()
            None => {
                // this should ideally never happen.
                // since we always pass the required history data.
                // In such case control should have been handed to opening_move

                // unreachable!()
                self.opening_move(game_context)
            }
        }
    }

    fn decide_second_move(
        &self,
        _game_context: &game::game::Game,
        opponent_decision: &Decision,
    ) -> game::decision::Decision {
        // return back the opponent last move
        opponent_decision.clone()
    }
}
