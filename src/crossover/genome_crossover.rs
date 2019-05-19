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
pub struct IntArrayCrossover;

impl Crossover for StringCrossover{
    type T = String;

    fn crossover(&mut self, _first_individual: Individual<String>, _second_individual: Individual<String>) -> Individual<String> {
        let mut new_individual= Individual::default();
        let gen_number = self.seed.gen::<f64>();

        if gen_number < self.crossover_rate {
            let len_of_individual =  &_first_individual.individual().to_string().chars().count();
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

//impl Crossover for IntArrayCrossover {
//    type T = ();
//
//    fn crossover<T: std::fmt::Display + ToString + Clone>(&mut self, _first_individual: Individual<T>, _second_individual: Individual<T>) -> Individual<T> {
//        _first_individual
//    }
//}

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
    point_locations.sort();
    point_locations
}

#[cfg(test)]
mod crossover_test {
    use crate::crossover::genome_crossover::Crossover;
    use crate::crossover::genome_crossover::StringCrossover;
    use crate::genome::population::Individual;

    #[test]
        fn test_string_crossover() {

        let seed: &[u8; 32] = &[1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3 ,1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];

        let individual = Individual::new(String::from("hello"), 5.0);
        let individual2 = Individual::new(String::from("hello"), 5.0);
        let mut string_crossover = StringCrossover::new(0.60, 3, *seed);
        let individual = string_crossover.crossover(individual, individual2);
        //println!("{}", individual);
    }
}
