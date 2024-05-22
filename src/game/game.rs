use super::decision::Decision;
use super::player;
use crate::game::scores;

pub type RoundOutcomes = std::collections::VecDeque<RoundOutcome>;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoundOutcome {
    pub player_one_move: Decision,
    pub player_two_move: Decision,
    pub player_one_point: scores::DefaultScores,
    pub player_two_point: scores::DefaultScores,
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

pub struct Game<PlayerOne, PlayerTwo> {
    // all the decision history
    pub decision_history: RoundOutcomes,
    // which round we are iterating on
    pub ongoing_round: RoundCount,
    // Player one
    pub player_one: PlayerOne,
    // Player two
    pub player_two: PlayerTwo,
}

impl<PlayerOne, PlayerTwo> Game<PlayerOne, PlayerTwo>
where
    PlayerOne: player::PlayerConcious<Self>,
    PlayerTwo: player::PlayerConcious<Self>,
{
    pub fn new(player_one: PlayerOne, player_two: PlayerTwo) -> Self {
        Game {
            decision_history: RoundOutcomes::new(),
            ongoing_round: 0,
            player_one,
            player_two,
        }
    }

    fn add_round_outcome(
        &mut self,
        game_config: &GameConfig,
        player_one_move: Decision,
        player_two_move: Decision,
    ) {
        let (point_one, point_two): (scores::DefaultScores, scores::DefaultScores) =
            Decision::get_scores(player_one_move, player_two_move);

        let outcome = RoundOutcome {
            player_one_move,
            player_two_move,
            player_one_point: self
                .decision_history
                .back()
                .map(|h| h.player_one_point + point_one)
                .unwrap_or(point_one),
            player_two_point: self
                .decision_history
                .back()
                .map(|h| h.player_two_point + point_two)
                .unwrap_or(point_two),
        };
        // keep decision_history variable in size bound
        if self.decision_history.len() >= game_config.outcome_buf_size {
            self.decision_history.pop_front();
        }
        self.decision_history.push_back(outcome);
    }

    pub fn play_one_round(&mut self, game_config: &GameConfig) {
        // increase current round by 1
        self.ongoing_round += 1;

        // construct a outcome
        let player_one_move = self.player_one.decide_first_move(&self);
        let player_two_move = self.player_two.decide_second_move(&self, &player_one_move);

        self.add_round_outcome(game_config, player_one_move, player_two_move);
    }

    pub fn play(&mut self, game_config: &GameConfig) {
        match game_config.max_rounds {
            Some(max_rounds) => {
                for _ in 0..max_rounds {
                    self.play_one_round(game_config)
                }
            }

            None => loop {
                self.play_one_round(game_config)
            },
        }
    }
}
