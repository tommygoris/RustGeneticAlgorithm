use crate::genome::population::ProblemType::Max;
use crate::genome::population::ProblemType::Min;

#[derive(Copy, Debug, Default)]
pub struct Individual<T> {
     individual : T,
     fitness : f64,
}

#[derive(Clone, Debug)]
pub struct Population<T> {
    list_of_individuals: Vec<Individual<T>>,
    top_fitness_individual: Individual<T>,
    problem_type: ProblemType,
}


impl <T> Individual<T> {

    pub fn new(individual : T, fitness : f64) -> Individual<T> {
        Individual {
            individual,
            fitness,
        }
    }

    pub fn individual(&self) -> &T {
        &self.individual
    }

    pub fn fitness(&self) -> &f64 {
        &self.fitness
    }

//    pub fn set_individual(&mut self, individual: T) {
//        self.individual = individual
//    }
//
//    pub fn set_fitness(&mut self, fitness: f64) {
//        self.fitness = fitness
//    }
}

impl<T: std::fmt::Display> std::fmt::Display for Individual<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Individual: {}, Fitness: {}", self.individual, self.fitness)
    }
}

impl<T> Clone for Individual<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ProblemType {
    Max,
    Min
}

//impl<T> Clone for Individual<T> {
//    fn clone(&self) -> Self {
//        &self.clone()
//    }
//
//    fn clone_from(&mut self, source: &Self) {
//        unimplemented!()
//    }
//}

impl <T> Population<T> {

    pub fn crossover(self: &Self) {

    }

    pub fn mutate(self: &Self) {

    }

    fn select_individual(self: &Self) {

    }

    pub fn list_of_individuals(&self) -> &Vec<Individual<T>> {&self.list_of_individuals}

    pub fn top_fitness_individual(&self) -> &Individual<T> {&self.top_fitness_individual}

    pub fn new(list_of_individuals: Vec<Individual<T>>, problem_type: ProblemType) -> Population<T> {
        Population {
            list_of_individuals,
            top_fitness_individual: list_of_individuals[0],
            problem_type
        }

    }

    pub fn find_top_individual(&self) -> &Individual<T> {
        let top_individual = &self.list_of_individuals()[0];
        for (index, individual) in self.list_of_individuals().iter().enumerate() {
//            match ProblemType {
//                Min =>
//                    if top_individual
//                Max =>
//            }

        }
        top_individual
    }
}

//impl<T: std::fmt::Display> std::fmt::Display for Population<T> {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//        write!(f, "Population: {}, Top Individual: {}", self.list_of_individuals, self.top_fitness_individual)
//    }
//}
