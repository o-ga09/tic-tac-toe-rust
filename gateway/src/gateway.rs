use usecase::port::usecase::Port;
use domain::domain::Board;
use domain::domain::Koma;
use ui::ui::GameUi;
use std::io::{Read};

pub struct GameGateway {
    ui: GameUi
}

impl Port for GameGateway {
    fn display(&self, board: Board) {
        self.ui.output(board.board);
    }

    fn input(&self, player: i32) -> Koma {
        let input: &mut dyn Read;
        let res: String = self.ui.input(input);
        let words: Vec<&str> = res.split_whitespace().collect();

        if words.len() != 2 {
            return Koma{order:player,x:-1,y:-1};
        }

        let x: i32 = words[0].parse().unwrap();
        let y: i32 = words[1].parse().unwrap();

        return Koma { order: player, x: x, y: y }
    }
}