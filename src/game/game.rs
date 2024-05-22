use super::decision::Decision;
use super::player;
use crate::game::scores;

pub type RoundOutcomes = std::collections::VecDeque<RoundOutcome>;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoundOutcome {
    pub player_one_move: Decision,
    pub player_two_move: Decision,
}

impl RoundOutcome {
    pub fn get_by_id(&self, player_id: super::player::PlayerId) -> &Decision {
        match player_id {
            player::PlayerId::PlayerOne => &self.player_one_move,
            player::PlayerId::PlayerTwo => &self.player_two_move,
        }
    }
}

pub type RoundCount = u64;
pub struct GameConfig {
    // How many round outcome should be kept in memory
    pub outcome_buf_size: usize,
    pub max_rounds: Option<RoundCount>,
}

pub struct Game {
    // all the decision history
    pub decision_history: RoundOutcomes,
    // which round we are iterating on
    pub ongoing_round: RoundCount,
    // how many times player one has own
    pub player_one_points: scores::DefaultScores,
    // how many times player two has won
    pub player_two_points: scores::DefaultScores,
}

impl Game {
    pub fn new() -> Self {
        Game {
            decision_history: RoundOutcomes::new(),
            ongoing_round: 0,
            player_one_points: 0,
            player_two_points: 0,
        }
    }

    fn add_round_outcome(
        &mut self,
        game_config: &GameConfig,
        player_one_move: Decision,
        player_two_move: Decision,
    ) {
        let outcome = RoundOutcome {
            player_one_move,
            player_two_move,
        };
        // keep decision_history variable in size bound
        if self.decision_history.len() >= game_config.outcome_buf_size {
            self.decision_history.pop_front();
        }
        self.decision_history.push_back(outcome);
    }

    pub fn play<PlayerOne, PlayerTwo>(
        &mut self,
        game_config: &GameConfig,
        player_one: PlayerOne,
        player_two: PlayerTwo,
    ) where
        PlayerOne: player::PlayerConcious<Self>,
        PlayerTwo: player::PlayerConcious<Self>,
    {
        let mut play_round = || {
            println!("=== Round {} ====", self.ongoing_round);

            // increase current round by 1
            self.ongoing_round += 1;

            // construct a outcome
            let player_one_move = player_one.decide_first_move(&self);
            let player_two_move = player_two.decide_second_move(&self, &player_one_move);
            let (point_one, point_two): (scores::DefaultScores, scores::DefaultScores) =
                Decision::get_scores(player_one_move, player_two_move);
            self.player_one_points += point_one;
            self.player_two_points += point_two;
            self.add_round_outcome(game_config, player_one_move, player_two_move);
            println!("Outcode: {:?}", self.decision_history.back());

            println!("===================")
        };

        match game_config.max_rounds {
            Some(max_rounds) => {
                for _ in 0..max_rounds {
                    play_round();
                }
            }

            None => loop {
                play_round();
            },
        }
    }
}
