use crate::logic::entities::board::Board;

pub trait BoardBuilder {
    fn build(&self) -> Board;
}