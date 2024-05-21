use std::ops::{Add, AddAssign};

pub trait Scores: AddAssign<Self> + AddAssign<u64> + Add<Self> + AddAssign<Self>
where
    Self: Sized,
{
    fn only_i_sniff() -> Self;
    fn we_both_cooperate() -> Self;
    fn we_both_sniff() -> Self;
    fn only_i_cooperate() -> Self;
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
