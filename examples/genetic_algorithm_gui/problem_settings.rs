use crate::Model;

pub trait ProblemSettings {
    fn on_start<'a>(&mut self);
    fn on_pause<'a>(&mut self);
    fn on_stop<'a>(&mut self);
}
