// Application layer - Command Pattern
use std::fmt::Debug;

pub trait Command: Debug {
    fn execute(&mut self);
    fn undo(&mut self);
}