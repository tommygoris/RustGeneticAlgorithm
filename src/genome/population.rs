use crate::crossover::genome_crossover::Crossover;
use crate::mutation::genome_mutation::Mutate;
use crate::selection::genome_selection::SelectIndividual;
use crate::genome::problem::FitnessFunction;

#[derive(Copy, Clone, Debug, Default)]
pub struct Individual<T> {
     individual : T,
     fitness : f64,
}

#[derive(Clone, Debug)]
pub struct Population<T>{
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
            problem_type,
        }
    }


    pub fn crossover(&mut self, crossover: &mut Crossover<T=T>, selector: &mut SelectIndividual<T=T>, mut fitness_function: Box<dyn FitnessFunction<T = T>>) {
        let mut new_population: Vec<Individual<T>> = Vec::new();
        for _ in self.list_of_individuals.iter() {
            let individual_one = selector.select_individual(self);
            let individual_two = selector.select_individual(self);
            let new_individual = crossover.crossover(&individual_one, &individual_two, &mut fitness_function);

            new_population.push(new_individual);
        }
        self.list_of_individuals = new_population;
    }

    pub fn mutate<'a>(&mut self, mutation: &'a mut dyn Mutate<T = T>, fitness_function: Box<dyn FitnessFunction<T = T>>) {
        self.list_of_individuals = mutation.mutate(self, fitness_function);
    }
    pub fn list_of_individuals(&self) -> &Vec<Individual<T>> { &self.list_of_individuals }

    pub fn find_top_individual(&mut self) -> &Individual<T> {
        let mut top_individual = &self.list_of_individuals()[0];
        for (_, individual) in self.list_of_individuals().iter().skip(1).enumerate() {
            match self.problem_type {
                ProblemType::Min =>
                    if top_individual.fitness > individual.fitness {
                        top_individual = individual;
                    }
                ProblemType::Max =>
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
