pub struct Board {
    pub board: [[u32;3];3]
}

pub struct Koma {
    pub order: i32,
    pub x: i32,
    pub y: i32
}

pub fn new() -> Board {
    Board { board: [[0;3];3] }
}

impl Board {
    pub fn init() -> Board {
        Board{
            board: [[0;3];3],
        }
    }
}
