use crate::genome::population::ProblemType::Max;
use crate::genome::population::ProblemType::Min;

#[derive(Copy, Clone, Debug, Default)]
pub struct Individual<T> {
     individual : T,
     fitness : f64,
}

#[derive(Clone, Debug)]
pub struct Population<T> {
    list_of_individuals: Vec<Individual<T>>,
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

#[derive(Copy, Clone, Debug)]
pub enum ProblemType {
    Max,
    Min
}

impl <T> Population<T> {

    pub fn new(list_of_individuals: Vec<Individual<T>>, problem_type: ProblemType) -> Population<T> {
        Population {
            list_of_individuals,
            problem_type
        }
    }


    pub fn crossover(self: &Self) {}

    pub fn mutate(self: &Self) {}

    fn select_individual(self: &Self) {}

    pub fn list_of_individuals(&self) -> &Vec<Individual<T>> { &self.list_of_individuals }

    pub fn find_top_individual(&mut self) -> &Individual<T> {
        let mut top_individual = &self.list_of_individuals()[0];
        for (index, individual) in self.list_of_individuals().iter().skip(1).enumerate() {
            match self.problem_type {
                Min =>
                    if top_individual.fitness > individual.fitness {
                        top_individual = individual;
                    }
                Max =>
                    if top_individual.fitness < individual.fitness {
                        top_individual = individual;
                }
            }
        }
        top_individual
    }

    pub fn problem_type(&self) -> ProblemType {
        self.problem_type
    }
}
