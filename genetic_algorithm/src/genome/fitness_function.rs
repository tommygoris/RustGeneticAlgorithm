pub trait FitnessFunction {
    type T;
    fn calculate_fitness(&mut self, _: &Self::T) -> f64;
}
