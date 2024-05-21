use super::scores;

/// one of this can be decided
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Decision {
    Cooperate,
    Sniff,
}

impl Decision {
    pub fn get_scores<Scores: scores::Scores + std::ops::AddAssign<u64>>(
        player_one_decision: Decision,
        player_two_decision: Decision,
    ) -> (Scores, Scores) {
        match (player_one_decision, player_two_decision) {
            (Decision::Cooperate, Decision::Cooperate) => {
                (Scores::we_both_cooperate(), Scores::we_both_cooperate())
            }

            (Decision::Sniff, Decision::Sniff) => {
                (Scores::we_both_sniff(), Scores::we_both_sniff())
            }

            (Decision::Cooperate, Decision::Sniff) => {
                (Scores::only_i_cooperate(), Scores::only_i_sniff())
            }

            (Decision::Sniff, Decision::Cooperate) => {
                (Scores::only_i_cooperate(), Scores::only_i_sniff())
            }
        }
    }
}
