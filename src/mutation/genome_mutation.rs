extern crate rand;
use crate::genome::population::{Population, Individual, ProblemType};
use rand::prelude::*;
use rand::rngs::StdRng;

pub trait Mutate {
    type T;
    fn mutate(&mut self, mut population: &Population<Self::T>) -> Population<String>;
}

pub struct StringMutation {
    mutation_rate: f64,
    possible_candidates: Vec<char>,
    seed: StdRng
}

impl Mutate for StringMutation {
    type T = String;

    fn mutate(&mut self, population: &Population<String>) -> Population<String> {
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

            new_population.push(Individual::new(mutated_individual, 6.0));
        }
        let new_pop = Population::new(new_population, population.problem_type());
        new_pop
    }
}

impl StringMutation {
    fn new(mutation_rate: f64, possible_candidates: Vec<char>, seed: [u8; 32]) -> StringMutation {
        StringMutation {
            mutation_rate,
            possible_candidates,
            seed: SeedableRng::from_seed(seed),
        }
    }
}

#[cfg(test)]
mod mutation_test {
    use crate::mutation::genome_mutation::{StringMutation, Mutate};
    use crate::genome::population::{Population, Individual, ProblemType};

    #[test]
    fn test_string_mutation() {
        let seed: &[u8; 32] = &[1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3 ,1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let possible_candidates = vec!['0','1'];

        let mut string_mutation = StringMutation::new(1.0, possible_candidates, *seed);


        let individual = Individual::new(String::from("un1111o"), 5.0);
        let individual2 = Individual::new(String::from("d12131314os"), 5.0);

        let list_of_individuals = vec![individual, individual2];

        let mut population = Population::new(list_of_individuals, ProblemType::Max);
        let new_pop = string_mutation.mutate(&population);
        assert_eq!(new_pop.list_of_individuals()[0].individual(), &String::from("1110000"));
        assert_eq!(new_pop.list_of_individuals()[1].individual(), &String::from("11101110101"));
    }
}