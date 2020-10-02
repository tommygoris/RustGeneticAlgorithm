use crate::genome::population::{Individual, ProblemType};
extern crate rand;

use crate::genome::fitness_function::FitnessFunction;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::string::ToString;

pub trait Crossover {
    type T;

    fn crossover(
        &mut self,
        first_individual: &Individual<Self::T>,
        second_individual: &Individual<Self::T>,
        fitness_function: &mut Box<dyn FitnessFunction<T = Self::T>>,
        problem_type: &ProblemType,
    ) -> Individual<Self::T>;
}

#[derive(Clone, Debug)]
pub struct StringCrossover {
    crossover_rate: f64,
    crossover_points: u32,
    seed: StdRng,
}
#[derive(Clone, Debug)]
pub struct VecIntegerCrossover {
    crossover_rate: f64,
    crossover_points: u32,
    seed: StdRng,
}

impl Crossover for StringCrossover {
    type T = String;

    fn crossover(
        &mut self,
        _first_individual: &Individual<String>,
        _second_individual: &Individual<String>,
        fitness_function: &mut Box<dyn FitnessFunction<T = String>>,
        problem_type: &ProblemType,
    ) -> Individual<String> {
        let gen_number = self.seed.gen::<f64>();

        if gen_number < self.crossover_rate {
            let len_of_individual = &_first_individual
                .retrieve_individual()
                .to_string()
                .chars()
                .count();
            println!(
                "start parent 1: {:?}, start parent 2: {:?}",
                _first_individual, _second_individual
            );
            if len_of_individual <= &usize::try_from(self.crossover_points).unwrap() {
                panic!(
                    "Please make your crossover points less than the problem length.
                        Current crossover points is: {} and current problem length is {}",
                    self.crossover_points, len_of_individual
                );
            }

            let points = get_crossover_locations(
                len_of_individual,
                self.crossover_points,
                self.seed.clone(),
            );

            let mut previous = 0;
            let mut new_string_individual = String::new();

            for (index, &location) in points.iter().enumerate() {
                let location = usize::try_from(location).unwrap();

                if index % 2 == 0 {
                    new_string_individual.push_str(
                        &_first_individual.retrieve_individual().to_string()[previous..location],
                    )
                } else {
                    new_string_individual.push_str(
                        &_second_individual.retrieve_individual().to_string()[previous..location],
                    )
                }
                previous = location;
            }
            let new_fitness = fitness_function.calculate_fitness(&new_string_individual);
            let new_individual = Individual::new(new_string_individual, new_fitness);
            println!("resulting child: {:?}", new_individual.clone());
            return new_individual;
        }

        return get_default_better_individual(_first_individual, _second_individual, &problem_type)
            .clone();
    }
}

impl StringCrossover {
    pub fn new(crossover_rate: f64, crossover_points: u32, seed: [u8; 32]) -> StringCrossover {
        StringCrossover {
            crossover_rate,
            crossover_points,
            seed: SeedableRng::from_seed(seed),
        }
    }
}

impl Crossover for VecIntegerCrossover {
    type T = Vec<u32>;

    fn crossover(
        &mut self,
        _first_individual: &Individual<Vec<u32>>,
        _second_individual: &Individual<Vec<u32>>,
        fitness_function: &mut Box<dyn FitnessFunction<T = Vec<u32>>>,
        problem_type: &ProblemType,
    ) -> Individual<Vec<u32>> {
        let gen_number = self.seed.gen::<f64>();

        if gen_number < self.crossover_rate {
            let len_of_individual = &_first_individual.retrieve_individual().len();

            if len_of_individual <= &usize::try_from(self.crossover_points).unwrap() {
                panic!(
                    "Please make your crossover points less than the problem length.
                        Current crossover points is: {} and current problem length is {}",
                    self.crossover_points, len_of_individual
                );
            }

            let points = get_crossover_locations(
                len_of_individual,
                self.crossover_points,
                self.seed.clone(),
            );

            let mut previous = 0;
            let mut new_vec_individual = Vec::new();

            for (index, &location) in points.iter().enumerate() {
                let location = usize::try_from(location).unwrap();
                if index % 2 == 0 {
                    new_vec_individual.extend_from_slice(
                        &_first_individual.retrieve_individual()[previous..location],
                    );
                } else {
                    new_vec_individual.extend_from_slice(
                        &_second_individual.retrieve_individual()[previous..location],
                    );
                }

                previous = location;
            }
            let new_fitness = fitness_function.calculate_fitness(&new_vec_individual);
            let new_individual = Individual::new(new_vec_individual, new_fitness);
            return new_individual;
        }

        return get_default_better_individual(_first_individual, _second_individual, problem_type)
            .clone();
    }
}

impl VecIntegerCrossover {
    fn new(crossover_rate: f64, crossover_points: u32, seed: [u8; 32]) -> VecIntegerCrossover {
        VecIntegerCrossover {
            crossover_rate,
            crossover_points,
            seed: SeedableRng::from_seed(seed),
        }
    }
}

fn get_crossover_locations(
    length_of_problem: &usize,
    crossover_points: u32,
    mut seed: StdRng,
) -> Vec<u32> {
    let mut point_locations = Vec::new();
    let mut set_locations = HashSet::new() as HashSet<u32>;
    while set_locations.len() < usize::try_from(crossover_points).unwrap() {
        let location = seed.gen_range(1, length_of_problem) as u32;
        if !set_locations.contains(&location) {
            set_locations.insert(location);
            point_locations.push(location)
        }
    }
    let length_of_problem = u32::try_from(*length_of_problem).unwrap();
    point_locations.push(length_of_problem);
    point_locations.sort();
    dbg!(point_locations.clone());
    point_locations
}

pub fn get_default_better_individual<'a, T>(
    indv_one: &'a Individual<T>,
    indv_two: &'a Individual<T>,
    problem_type: &ProblemType,
) -> &'a Individual<T> {
    match problem_type {
        ProblemType::Max => {
            if indv_one.fitness() > indv_two.fitness() {
                indv_one
            } else {
                indv_two
            }
        }
        ProblemType::Min => {
            if indv_one.fitness() > indv_two.fitness() {
                indv_two
            } else {
                indv_one
            }
        }
    }
}

#[cfg(test)]
mod crossover_test {
    use crate::crossover::genome_crossover::StringCrossover;
    use crate::crossover::genome_crossover::VecIntegerCrossover;
    use crate::crossover::genome_crossover::{get_default_better_individual, Crossover};
    use crate::genome::fitness_function::FitnessFunction;
    use crate::genome::population::{Individual, ProblemType};
    use std::borrow::Borrow;

    #[derive(Default, Copy, Clone, Debug)]
    struct TestStringFitnessFunction;
    #[derive(Default, Copy, Clone, Debug)]
    struct TestVecFitnessFunction;
    impl FitnessFunction for TestStringFitnessFunction {
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

    impl FitnessFunction for TestVecFitnessFunction {
        type T = Vec<u32>;
        fn calculate_fitness(&mut self, individual: &Vec<u32>) -> f64 {
            let mut fitness = 0.0;
            for char in individual {
                if char.eq(&1) {
                    fitness += 1.0;
                }
            }
            fitness
        }
    }
    #[test]
    fn test_string_crossover() {
        let seed: &[u8; 32] = &[
            1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
            2, 3, 4,
        ];
        let mut fitness_function: Box<dyn FitnessFunction<T = String>> =
            Box::new(TestStringFitnessFunction::default());
        let individual = Individual::new(String::from("uno"), 5.0);
        let individual2 = Individual::new(String::from("dos"), 5.0);
        let mut string_crossover = StringCrossover::new(1.0, 2, *seed);
        let individual = string_crossover.crossover(
            &individual,
            &individual2,
            &mut fitness_function,
            &ProblemType::Max,
        );
        assert_eq!(individual.retrieve_individual(), &String::from("uoo"));

        let mut string_crossover = StringCrossover::new(1.0, 13, *seed);
        let individual = Individual::new(String::from("10101010101010"), 5.0);
        let individual2 = Individual::new(String::from("01010101010101"), 5.0);
        let individual = string_crossover.crossover(
            &individual,
            &individual2,
            &mut fitness_function,
            &ProblemType::Max,
        );
        assert_eq!(
            individual.retrieve_individual(),
            &String::from("11111111111111")
        );
    }

    #[test]
    fn test_string_crossover_skip_crossover() {
        let seed: &[u8; 32] = &[
            1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
            2, 3, 4,
        ];
        let mut fitness_function: Box<dyn FitnessFunction<T = String>> =
            Box::new(TestStringFitnessFunction::default());
        let mut individual = Individual::new(String::from("uno"), 5.0);
        let individual2 = Individual::new(String::from("dos"), 6.0);
        let mut string_crossover = StringCrossover::new(0.0, 2, *seed);
        let mut individual3 = string_crossover.crossover(
            &individual,
            &individual2,
            &mut fitness_function,
            &ProblemType::Max,
        );

        assert_eq!(individual3.retrieve_individual(), &String::from("dos"));

        let mut string_crossover = StringCrossover::new(0.0, 13, *seed);
        let individual = Individual::new(String::from("10101010101010"), 5.0);
        let individual2 = Individual::new(String::from("01010101010101"), 5.0);
        let mut individual3 = string_crossover.crossover(
            &individual,
            &individual2,
            &mut fitness_function,
            &ProblemType::Max,
        );

        assert_eq!(
            individual3.retrieve_individual(),
            &String::from("01010101010101")
        );
        //println!("{}", individual);
    }

    #[test]
    fn test_vec_integer_crossover() {
        let seed: &[u8; 32] = &[
            1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
            2, 3, 4,
        ];
        let mut fitness_function: Box<dyn FitnessFunction<T = Vec<u32>>> =
            Box::new(TestVecFitnessFunction::default());

        let mut vec_crossover = VecIntegerCrossover::new(1.0, 2, *seed);
        let individual = Individual::new(vec![1, 2, 3], 5.0);
        let individual2 = Individual::new(vec![4, 5, 6], 5.0);
        let individual = vec_crossover.crossover(
            &individual,
            &individual2,
            &mut fitness_function,
            &ProblemType::Max,
        );

        assert_eq!(individual.retrieve_individual(), &vec![1, 5, 3]);
    }
}
