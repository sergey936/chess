use crate::logic::entities::board::Board;

pub trait BoardBuilder {
    fn new(&self) -> Board;
}