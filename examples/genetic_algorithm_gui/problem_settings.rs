use crate::Model;

pub trait ProblemSettings {
    fn on_start<'a>(&mut self, model: &Model);
    fn on_pause<'a>(&mut self, model: &Model);
    fn on_stop<'a>(&mut self, model: &Model);
}
