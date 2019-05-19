use crate::genome::population::Individual;
extern crate rand;

use rand::prelude::*;
use rand::rngs::StdRng;
use std::string::ToString;
use std::collections::HashSet;
use std::convert::TryFrom;

pub trait Crossover {
    type T;

    fn crossover(&mut self, first_individual: Individual<Self::T>, second_individual: Individual<Self::T>) -> Individual<Self::T>;
}


pub struct StringCrossover {
    crossover_rate: f64,
    crossover_points: u32,
    seed: StdRng
}
pub struct VecIntegerCrossover {
    crossover_rate: f64,
    crossover_points: u32,
    seed: StdRng
}

impl Crossover for StringCrossover{
    type T = String;

    fn crossover(&mut self, _first_individual: Individual<String>, _second_individual: Individual<String>) -> Individual<String> {
        let mut new_individual: Individual<String> = Individual::default();
        let gen_number = self.seed.gen::<f64>();

        if gen_number < self.crossover_rate {
            let len_of_individual =  &_first_individual.individual().to_string().chars().count();

            if len_of_individual <= &usize::try_from(self.crossover_points).unwrap() {
                panic!("Please make your crossover points less than the problem length.
                        Current crossover points is: {} and current problem length is {}", self.crossover_points, len_of_individual);
            }


            let points = get_crossover_locations(len_of_individual, self.crossover_points, self.seed.clone());

            let mut previous = 0;
            let mut new_string_individual = String::new();

            for (index, &location) in points.iter().enumerate() {
                let location = usize::try_from(location).unwrap();


                if index%2 == 0 {
                    new_string_individual.push_str(&_first_individual.individual().to_string()[previous..location])
                }

                else {
                    new_string_individual.push_str(&_second_individual.individual().to_string()[previous..location])
                }
                previous = location;

            }
            new_individual = Individual::new(new_string_individual, 6.0);
            return new_individual
        }

        let new_individual: Individual<String> =
            if _first_individual.fitness() > _second_individual.fitness() {
                _first_individual
            }
            else {
                _second_individual
            };
        new_individual
    }
}

impl StringCrossover {
    fn new(crossover_rate: f64, crossover_points: u32, seed: [u8; 32]) -> StringCrossover {
        StringCrossover {
            crossover_rate,
            crossover_points,
            seed: SeedableRng::from_seed(seed),
        }
    }
}

impl Crossover for VecIntegerCrossover {
    type T = Vec<u32>;

    fn crossover(&mut self, _first_individual: Individual<Vec<u32>>, _second_individual: Individual<Vec<u32>>) -> Individual<Vec<u32>> {
        let mut new_individual: Individual<Vec<u32>> = Individual::default();
        let gen_number = self.seed.gen::<f64>();

        if gen_number < self.crossover_rate {
            let len_of_individual = &_first_individual.individual().len();

            if len_of_individual <= &usize::try_from(self.crossover_points).unwrap() {
                panic!("Please make your crossover points less than the problem length.
                        Current crossover points is: {} and current problem length is {}", self.crossover_points, len_of_individual);
            }


            let points = get_crossover_locations(len_of_individual, self.crossover_points, self.seed.clone());

            let mut previous = 0;
            let mut new_vec_individual = Vec::new();

            for (index, &location) in points.iter().enumerate() {
                let location = usize::try_from(location).unwrap();
                if index%2 == 0 {
                    new_vec_individual.extend_from_slice(&_first_individual.individual()[previous..location]);
                }
                else {
                    new_vec_individual.extend_from_slice(&_second_individual.individual()[previous..location]);
                }

                previous = location;


            }
            new_individual = Individual::new(new_vec_individual, 6.0);
            return new_individual
        }
        let new_individual: Individual<Vec<u32>> =
            if _first_individual.fitness() > _second_individual.fitness() {
                _first_individual
            }
            else {
                _second_individual
            };
        new_individual
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



fn get_crossover_locations(length_of_problem: &usize, crossover_points: u32, mut seed : StdRng) -> Vec<u32> {
    let mut point_locations  = Vec::new();
    let mut set_locations = HashSet::new() as HashSet<u32>;
    while set_locations.len() < usize::try_from(crossover_points).unwrap() {
        let location: u32  = seed.gen_range(1, length_of_problem) as u32;
        if !set_locations.contains(&location) {
            set_locations.insert(location);
            point_locations.push(location)
        }
    }
    let length_of_problem = u32::try_from(*length_of_problem).unwrap();
    point_locations.push(length_of_problem);
    point_locations.sort();
    point_locations
}

#[cfg(test)]
mod crossover_test {
    use crate::crossover::genome_crossover::Crossover;
    use crate::crossover::genome_crossover::StringCrossover;
    use crate::crossover::genome_crossover::VecIntegerCrossover;
    use crate::genome::population::Individual;

        #[test]
        fn test_string_crossover() {

        let seed: &[u8; 32] = &[1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3 ,1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];

        let individual = Individual::new(String::from("uno"), 5.0);
        let individual2 = Individual::new(String::from("dos"), 5.0);
        let mut string_crossover = StringCrossover::new(0.60, 2, *seed);
        let individual = string_crossover.crossover(individual, individual2);
        assert_eq!(individual.individual(), &String::from("uoo"));

        let mut string_crossover = StringCrossover::new(1.0, 13, *seed);
        let individual = Individual::new(String::from("10101010101010"), 5.0);
        let individual2 = Individual::new(String::from("01010101010101"), 5.0);
        let individual = string_crossover.crossover(individual, individual2);
        assert_eq!(individual.individual(), &String::from("11111111111111"));
        println!("{}", individual);
    }

    #[test]
    fn test_vec_integer_crossover() {
        let seed: &[u8; 32] = &[1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3 ,1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];

        let mut vec_crossover = VecIntegerCrossover::new(1.0, 2, *seed);
        let individual = Individual::new(vec![1, 2, 3], 5.0);
        let individual2 = Individual::new(vec![4, 5, 6], 5.0);
        let individual = vec_crossover.crossover(individual, individual2);
        assert_eq!(individual.individual(), &vec![1, 5, 3]);
    }
}
