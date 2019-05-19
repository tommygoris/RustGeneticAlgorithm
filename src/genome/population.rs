#[derive(Copy, Clone, Debug, Default)]
pub struct Individual<T> {
     individual : T,
     fitness : f64,
}

pub struct Population<T> {
    list_of_individuals: Vec<T>
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

}