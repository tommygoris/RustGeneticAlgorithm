pub trait FitnessFunction {
    type T;
    fn calculate_fitness(&mut self, _: &Self::T) -> f64;
}

#[derive(Default, Copy, Clone, Debug)]
pub struct OneMax;

impl FitnessFunction for OneMax {
    type T = String;

    fn calculate_fitness(&mut self, individual: &String) -> f64 {
        let mut fitness = 0.0;
        for char in individual.chars() {
            if char.eq(&'1') {
                fitness += 1.0;
            }
        }
        fitness
    }
}
