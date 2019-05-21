
pub trait Mutate {
    type T;
    fn mutate(&mut self, mut population: &Population<Self::T>) -> Individual<Self::T>;
}

pub struct TournamentSelection {
    k_value: u32,
    percent: f64,
}