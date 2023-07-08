use domain::domain::Board;
use domain::domain::Koma;
use ui::ui::GameUi;
use usecase::port::usecase::InputPort;
use usecase::port::usecase::OutputPort;
use std::io::stdin;
use std::io::{Read};

pub struct GameGateway {
    ui: GameUi
}

impl OutputPort for GameGateway {
    fn display(&self, board: Board) {
        let _ = self.ui.output(&board.board);
    }
}

impl InputPort for GameGateway {
    fn input(&self, player: i32) -> Koma {
        let stdin = stdin();
        let input: &mut dyn Read = &mut stdin.lock();
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