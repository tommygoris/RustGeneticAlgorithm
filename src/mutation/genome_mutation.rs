extern crate rand;
use crate::genome::population::Population;
use rand::prelude::*;
use rand::rngs::StdRng;

pub trait Mutate {
    type T;
    fn mutate(&mut self, population: &Population<Self::T>);
}

pub struct StringMutation {
    mutation_rate: f64,
    possible_candidates: Vec<char>,
    seed: StdRng
}

impl Mutate for StringMutation {
    type T = String;

    fn mutate(&mut self, population: &Population<String>) {
        for (index, individual) in population.list_of_individuals().iter().enumerate() {

        }
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


        let individual = Individual::new(String::from("uno"), 5.0);
        let individual2 = Individual::new(String::from("dos"), 5.0);

        let list_of_individuals = vec![individual, individual2];

        let population = Population::new(list_of_individuals, ProblemType::Max);
        string_mutation.mutate(&population);
        println!("{:?}", population);
    }
}