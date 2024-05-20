use super::scores;

/// one of this can be decided
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Decision {
    Cooperate,
    Sniff,
}

impl Decision {
    pub fn set_scores<Scores: scores::Scores + std::ops::AddAssign<u64>>(
        player_one: (Decision, &mut Scores),
        player_two: (Decision, &mut Scores),
    ) {
        match (player_one.0, player_two.0) {
            (Decision::Cooperate, Decision::Cooperate) => {
                *player_one.1 += Scores::we_both_cooperate();
                *player_two.1 += Scores::we_both_cooperate();
            }

            (Decision::Sniff, Decision::Sniff) => {
                *player_one.1 += Scores::we_both_sniff();
                *player_two.1 += Scores::we_both_sniff();
            }

            (Decision::Cooperate, Decision::Sniff) => {
                *player_one.1 += Scores::only_i_cooperate();
                *player_two.1 += Scores::only_i_sniff();
            }

            (Decision::Sniff, Decision::Cooperate) => {
                *player_one.1 += Scores::only_i_sniff();
                *player_two.1 += Scores::only_i_cooperate()
            }
        }
    }
}
