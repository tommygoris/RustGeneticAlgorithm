use crate::genome::population::{Individual, Population, ProblemType};
use rand::prelude::*;
use rand::rngs::StdRng;

pub trait SelectIndividual {
    type T;
    fn select_individual(&mut self, population: &Population<Self::T>) -> Individual<Self::T>;
}
#[derive(Clone, Debug)]
pub struct TournamentSelection {
    k_value: u32,
    stronger_individual_win_chance: f64,
    seed: StdRng,
}
// TODO Figure out if we want to remove chosen individuals from the population if they have already been selected. Mostly Critical when populations are extremely small.
impl SelectIndividual for TournamentSelection {
    type T = String;
    fn select_individual(&mut self, population: &Population<String>) -> Individual<String> {
        let population_amount = population.list_of_individuals().len();

        let location = self.seed.gen_range(0, population_amount);

        let mut chosen_individual = &population.list_of_individuals()[location];

        for _individual_number in 0..self.k_value - 1 {
            let location = self.seed.gen_range(0, population_amount);

            let individual = &population.list_of_individuals()[location];

            let gen_number = self.seed.gen::<f64>();
            match population.problem_type() {
                ProblemType::Min => {
                    if chosen_individual.fitness() < individual.fitness() {
                        if self.stronger_individual_win_chance < gen_number {
                            chosen_individual = individual;
                        }
                    } else if chosen_individual.fitness() > individual.fitness() {
                        if self.stronger_individual_win_chance > gen_number {
                            chosen_individual = individual;
                        }
                    }
                }

                ProblemType::Max => {
                    if chosen_individual.fitness() > individual.fitness() {
                        if self.stronger_individual_win_chance < gen_number {
                            chosen_individual = individual;
                        }
                    } else if chosen_individual.fitness() < individual.fitness() {
                        if self.stronger_individual_win_chance > gen_number {
                            chosen_individual = individual;
                        }
                    }
                }
            }
        }
        chosen_individual.clone()
    }
}

impl TournamentSelection {
    pub fn new(
        k_value: u32,
        stronger_individual_win_chance: f64,
        seed: [u8; 32],
    ) -> TournamentSelection {
        TournamentSelection {
            k_value,
            stronger_individual_win_chance,
            seed: SeedableRng::from_seed(seed),
        }
    }
}

#[cfg(test)]
mod selection_test {
    use crate::genome::population::{Individual, Population, ProblemType};
    use crate::selection::genome_selection::{SelectIndividual, TournamentSelection};

    #[test]
    fn test_individual_selection() {
        let seed: &[u8; 32] = &[
            1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
            2, 3, 4,
        ];

        let individual = Individual::new(String::from("00000"), 6.0);
        let individual2 = Individual::new(String::from("11111"), 5.0);

        let list_of_individuals = vec![individual, individual2];

        let mut population = Population::new(list_of_individuals, ProblemType::Max);

        let mut tournament_selection = TournamentSelection::new(2, 1.0, *seed);

        let individual3 = tournament_selection.select_individual(&population);

        assert_eq!(individual3.individual(), &String::from("00000"));

        let mut tournament_selection = TournamentSelection::new(2, 0.0, *seed);
        let individual3 = tournament_selection.select_individual(&population);
        assert_eq!(individual3.individual(), &String::from("11111"));

        let individual = Individual::new(String::from("00000"), 6.0);
        let individual2 = Individual::new(String::from("11111"), 5.0);
        let list_of_individuals = vec![individual, individual2];

        let mut population = Population::new(list_of_individuals, ProblemType::Min);
        let mut tournament_selection = TournamentSelection::new(2, 1.0, *seed);
        let individual3 = tournament_selection.select_individual(&population);
        assert_eq!(individual3.individual(), &String::from("11111"));

        let mut tournament_selection = TournamentSelection::new(2, 0.0, *seed);
        let individual3 = tournament_selection.select_individual(&population);
        assert_eq!(individual3.individual(), &String::from("00000"));
    }
}
