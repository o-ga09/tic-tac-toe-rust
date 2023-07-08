use crate::port::usecase::InputPort;
use crate::port::usecase::OutputPort;
use domain::domain::Koma;

pub fn iswin(board: &Vec<Vec<String>>) -> bool {
    if check_vertical(&board) || check_horizon(&board) || check_cross(&board) {
        return true;
    }

    return false;
}

pub fn display(game_port: &impl OutputPort, board: Vec<Vec<String>>) {
    game_port.display(domain::domain::Board { board: board });
}

pub fn input(game_port: &impl InputPort, player: i32) -> Koma {
    return game_port.input(player);
}

fn check_vertical(board: &Vec<Vec<String>>) -> bool {
    for row in board {
        println!("{:?}", row);
    }
    for i in 0..3 {
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] == board[i][2] && board[0][0] != "-" {
            return true;
        }
    }

    return false;
}

fn check_horizon(board: &Vec<Vec<String>>) -> bool {
    for i in 0..3 {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] == board[2][i] && board[0][0] != "-" {
            return true;
        }
    }

    return false;
}

fn check_cross(board: &Vec<Vec<String>>) -> bool {
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] == board[2][2] && board[0][0] != "-" {
        return true;
    } else if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] == board[2][0] && board[0][2] != "-" {
        return true;
    }

    return false;
}

// fn is_empty(board: Vec<Vec<String>>, x:usize, y:usize) -> bool {
//     return board[x][y] == "-"
// }

#[cfg(test)]
mod tests {
    use crate::port::usecase::{MockOutputPort, MockInputPort};
    use domain::domain::{Board};

    use super::*;
    use mockall::{predicate::eq};    

    #[test]
    fn test_iswin(){
        let board = vec![
            vec!["○".to_string(), "○".to_string(), "○".to_string()],
            vec!["-".to_string(), "-".to_string(), "-".to_string()],
            vec!["-".to_string(), "-".to_string(), "-".to_string()],
        ];
        let res = iswin(&board);
        assert_eq!(res,true);
    }

    #[test]
    fn test_input() {
        let mut mock = MockInputPort::new();
        let player:i32 = 1;

        mock.expect_input()
            .with(eq(player))
            .return_const(domain::domain::Koma{order: 1,x: 1, y: 1})
            .times(1);

        let res = input(&mock, player);
        assert_eq!(res.order,1);
        assert_eq!(res.x,1);
        assert_eq!(res.y,1);
    }

    #[test]
    fn test_display(){
        let mut mock = MockOutputPort::new();
        let board = vec![];
        let b = board.clone();
        let mock_board : Board = domain::domain::Board{ board };
        mock.expect_display()
            .with(eq(mock_board))
            .times(1);

        display(&mock, b);
    }

}