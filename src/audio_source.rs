use std::fmt::Debug;

pub trait AudioSource: Send + Debug {
    fn next_sample(&mut self) -> f32;
    fn is_finished(&self) -> bool;
    fn release(&mut self);
}
