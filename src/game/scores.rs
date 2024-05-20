pub trait Scores {
    fn only_i_sniff() -> u64;
    fn we_both_cooperate() -> u64;
    fn we_both_sniff() -> u64;
    fn only_i_cooperate() -> u64;
}

pub type DefaultScores = u64;
impl Scores for DefaultScores {
    fn only_i_sniff() -> u64 {
        3
    }
    fn we_both_cooperate() -> u64 {
        2
    }
    fn we_both_sniff() -> u64 {
        1
    }
    fn only_i_cooperate() -> u64 {
        0
    }
}
