use domain::domain::Board;
use domain::domain::Koma;

pub trait Port {
    fn display(&self, board: Board);
    fn input(&self, player: i32) -> Koma;
}