use domain::domain::Board;
use domain::domain::Koma;
use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait InputPort {
    fn input(&self, player: i32) -> Koma;
}

#[automock]
pub trait OutputPort {
    fn display(&self, board: Board);
}