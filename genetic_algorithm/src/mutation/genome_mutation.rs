extern crate rand;
use crate::genome::fitness_function::FitnessFunction;
use crate::genome::population::{Individual, Population};
use rand::prelude::*;
use rand::rngs::StdRng;

pub trait Mutate {
    type T;
    fn mutate(
        &mut self,
        population: &Population<Self::T>,
        mut fitness_function: Box<dyn FitnessFunction<T = Self::T>>,
    ) -> Vec<Individual<Self::T>>;
}

#[derive(Clone, Debug)]
pub struct StringMutation {
    mutation_rate: f64,
    possible_candidates: Vec<char>,
    seed: StdRng,
}

#[derive(Clone, Debug)]
pub struct VecIntegerMutation {
    mutation_rate: f64,
    possible_candidates: Vec<u32>,
    seed: StdRng,
}

impl Mutate for StringMutation {
    type T = String;

    fn mutate(
        &mut self,
        population: &Population<String>,
        mut fitness_function: Box<dyn FitnessFunction<T = String>>,
    ) -> Vec<Individual<String>> {
        let mut new_population: Vec<Individual<String>> = Vec::new();
        for individual in population.list_of_individuals().iter() {
            let mut mutated_individual = String::new();
            for string_individual_char in individual.individual().chars() {
                let gen_number = self.seed.gen::<f64>();
                if gen_number < self.mutation_rate {
                    let location = self.seed.gen_range(0, self.possible_candidates.len());

                    mutated_individual.push(self.possible_candidates[location]);
                } else {
                    mutated_individual.push(string_individual_char);
                }
            }
            let new_fitness = fitness_function.calculate_fitness(&mutated_individual);
            new_population.push(Individual::new(mutated_individual, new_fitness));
        }
        new_population
    }
}

impl StringMutation {
    pub fn new(
        mutation_rate: f64,
        possible_candidates: Vec<char>,
        seed: [u8; 32],
    ) -> StringMutation {
        StringMutation {
            mutation_rate,
            possible_candidates,
            seed: SeedableRng::from_seed(seed),
        }
    }
}

impl Mutate for VecIntegerMutation {
    type T = Vec<u32>;

    fn mutate(
        &mut self,
        population: &Population<Vec<u32>>,
        mut fitness_function: Box<dyn FitnessFunction<T = Vec<u32>>>,
    ) -> Vec<Individual<Vec<u32>>> {
        let mut new_population: Vec<Individual<Vec<u32>>> = Vec::new();
        for individual in population.list_of_individuals().iter() {
            let mut mutated_individual = Vec::new();
            for int_item in individual.individual().iter() {
                let gen_number = self.seed.gen::<f64>();
                if gen_number < self.mutation_rate {
                    let location = self.seed.gen_range(0, self.possible_candidates.len());

                    mutated_individual.push(self.possible_candidates[location]);
                } else {
                    mutated_individual.push(*int_item);
                }
            }

            let new_fitness = fitness_function.calculate_fitness(&mutated_individual);
            new_population.push(Individual::new(mutated_individual, new_fitness));
        }
        new_population
    }
}

impl VecIntegerMutation {
    fn new(
        mutation_rate: f64,
        possible_candidates: Vec<u32>,
        seed: [u8; 32],
    ) -> VecIntegerMutation {
        VecIntegerMutation {
            mutation_rate,
            possible_candidates,
            seed: SeedableRng::from_seed(seed),
        }
    }
}

#[cfg(test)]
mod mutation_test {
    use crate::genome::population::{Individual, Population, ProblemType};
    use crate::mutation::genome_mutation::{Mutate, StringMutation, VecIntegerMutation};

    #[test]
    fn test_string_mutation() {
        let seed: &[u8; 32] = &[
            1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
            2, 3, 4,
        ];
        let possible_candidates = vec!['0', '1'];

        let mut string_mutation = StringMutation::new(1.0, possible_candidates, *seed);

        let individual = Individual::new(String::from("un1111o"), 5.0);
        let individual2 = Individual::new(String::from("d12131314os"), 5.0);

        let list_of_individuals = vec![individual, individual2];

        let mut population = Population::new(list_of_individuals, ProblemType::Max);
        let new_pop = string_mutation.mutate(&population);
        assert_eq!(new_pop[0].individual(), &String::from("1110000"));
        assert_eq!(new_pop[1].individual(), &String::from("11101110101"));
    }

    #[test]
    fn test_vec_int_mutation() {
        let seed: &[u8; 32] = &[
            1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
            2, 3, 4,
        ];
        let possible_candidates = vec![2, 3];

        let mut vec_int_mutation = VecIntegerMutation::new(1.0, possible_candidates, *seed);

        let individual = Individual::new(vec![0, 0, 0, 0, 0], 5.0);
        let individual2 = Individual::new(vec![1, 1, 1, 1, 1], 5.0);

        let list_of_individuals = vec![individual, individual2];

        let mut population = Population::new(list_of_individuals, ProblemType::Max);
        let new_pop = vec_int_mutation.mutate(&population);

        assert_eq!(new_pop[0].individual(), &vec![3, 3, 3, 2, 2]);
        assert_eq!(new_pop[1].individual(), &vec![2, 2, 3, 3, 3]);
    }
}
