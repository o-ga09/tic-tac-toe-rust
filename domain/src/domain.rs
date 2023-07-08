#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub board: Vec<Vec<String>>
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Koma {
    pub order: i32,
    pub x: i32,
    pub y: i32
}

impl Board {
    pub fn new() -> Board {
        let board = vec![
            vec!["-".to_string(), "-".to_string(), "-".to_string()],
            vec!["-".to_string(), "-".to_string(), "-".to_string()],
            vec!["-".to_string(), "-".to_string(), "-".to_string()],
        ];

        Board { board }
    }
}
