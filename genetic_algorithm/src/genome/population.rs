use crate::crossover::genome_crossover::Crossover;
use crate::genome::fitness_function::FitnessFunction;
use crate::mutation::genome_mutation::Mutate;
use crate::selection::genome_selection::SelectIndividual;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Individual<T> {
    individual: T,
    pub fitness: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Population<T> {
    list_of_individuals: Vec<Individual<T>>,
    problem_type: ProblemType,
}

impl<T> Individual<T> {
    pub fn new(individual: T, fitness: f64) -> Individual<T> {
        Individual {
            individual,
            fitness,
        }
    }

    pub fn retrieve_individual(&self) -> &T {
        &self.individual
    }
    pub fn retrieve_individual_mut(&mut self) -> &mut T {
        &mut self.individual
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
        write!(
            f,
            "Individual: {}, Fitness: {}",
            self.individual, self.fitness
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ProblemType {
    Max,
    Min,
}

impl<T: Clone + Serialize + Deserialize> Population<T> {
    pub fn new(
        list_of_individuals: Vec<Individual<T>>,
        problem_type: ProblemType,
    ) -> Population<T> {
        Population {
            list_of_individuals,
            problem_type,
        }
    }

    pub fn new_from_file(file_path: &'static str) -> Population<T> {
        serde_json::from_str(file_path).expect("Failed to read file")
    }

    pub fn crossover(
        &mut self,
        crossover: &mut dyn Crossover<T = T>,
        selector: &mut dyn SelectIndividual<T>,
        mut fitness_function: Box<dyn FitnessFunction<T = T>>,
    ) {
        let mut new_population: Vec<Individual<T>> = Vec::new();
        for _ in self.list_of_individuals.iter() {
            let individual_one = selector.select_individual(self);
            let individual_two = selector.select_individual(self);
            let new_individual = crossover.crossover(
                &individual_one,
                &individual_two,
                &mut fitness_function,
                &self.problem_type,
            );

            new_population.push(new_individual);
        }
        self.list_of_individuals = new_population;
    }

    pub fn mutate<'a>(
        &mut self,
        mutation: &'a mut dyn Mutate<T = T>,
        fitness_function: Box<dyn FitnessFunction<T = T>>,
    ) {
        self.list_of_individuals = mutation.mutate(self, fitness_function);
    }

    pub fn save_to_file(&mut self, file_path: &str) {
        serde_json::to_writer(&File::create(file_path).unwrap(), &self).unwrap()
    }

    pub fn list_of_individuals(&self) -> &Vec<Individual<T>> {
        &self.list_of_individuals
    }

    pub fn list_of_individuals_mut(&mut self) -> &mut Vec<Individual<T>> {
        &mut self.list_of_individuals
    }

    pub fn find_top_individual(&mut self) -> &Individual<T> {
        let mut top_individual = &self.list_of_individuals()[0];
        for (_, individual) in self.list_of_individuals().iter().skip(1).enumerate() {
            match self.problem_type {
                ProblemType::Min => {
                    if top_individual.fitness > individual.fitness {
                        top_individual = individual;
                    }
                }
                ProblemType::Max => {
                    if top_individual.fitness < individual.fitness {
                        top_individual = individual;
                    }
                }
            }
        }
        top_individual
    }

    pub fn find_top_individual_mut(&mut self) -> &mut Individual<T> {
        let problem_type = self.problem_type;
        self.list_of_individuals_mut()
            .iter_mut()
            .fold_first(|max, val| match problem_type {
                ProblemType::Max => {
                    if val.fitness > max.fitness {
                        val
                    } else {
                        max
                    }
                }
                ProblemType::Min => {
                    if val.fitness < max.fitness {
                        val
                    } else {
                        max
                    }
                }
            })
            .unwrap()
    }

    pub fn problem_type(&self) -> ProblemType {
        self.problem_type
    }

    pub fn print_pop(&mut self)
    where
        T: std::fmt::Debug,
    {
        for (index, indv) in self.list_of_individuals.iter().enumerate() {
            println!("Individual {} {:?}", index, indv);
        }
    }
}

#[cfg(test)]
mod population_test {
    use crate::genome::population::{Individual, Population, ProblemType};

    fn create_list_of_individuals() -> Vec<Individual<String>> {
        let num_of_indvs = 10;
        let mut list_of_indvs = Vec::new();
        let mut str = "".to_owned();
        let mut start_fitness = 0.0;

        for _ in 0..num_of_indvs {
            str.push_str("1");
            start_fitness += 1.0;
            list_of_indvs.push(Individual::new(str.clone(), start_fitness));
        }
        list_of_indvs
    }

    #[test]
    fn find_top_individual() {
        let list_of_indvs = create_list_of_individuals();
        let mut population = Population::new(list_of_indvs.clone(), ProblemType::Max);
        let top_individual_mut = population.find_top_individual_mut();

        assert_eq!("1111111111", top_individual_mut.individual);
        assert_eq!(10.0, top_individual_mut.fitness);

        let mut population = Population::new(list_of_indvs.clone(), ProblemType::Min);
        let top_individual_mut = population.find_top_individual_mut();

        assert_eq!("1", top_individual_mut.individual);
        assert_eq!(1.0, top_individual_mut.fitness);

        let mut population = Population::new(list_of_indvs.clone(), ProblemType::Max);
        let top_individual = population.find_top_individual();

        assert_eq!("1111111111", top_individual.individual);
        assert_eq!(10.0, top_individual.fitness);

        let mut population = Population::new(list_of_indvs.clone(), ProblemType::Min);
        let top_individual = population.find_top_individual();

        assert_eq!("1", top_individual.individual);
        assert_eq!(1.0, top_individual.fitness);
    }

    #[test]
    fn save_and_load_to_file() {
        let population_test = "unit_test_population.json";
        let list_of_indvs = create_list_of_individuals();

        let mut population = Population::new(list_of_indvs.clone(), ProblemType::Max);
        population.save_to_file(population_test);

        let mut population = Population::<String>::new_from_file(population_test);
    }
}
