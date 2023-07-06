pub fn iswin(board: &Vec<Vec<String>>) -> bool {
    if check_vertical(&board) || check_horizon(&board) || check_cross(&board) {
        return true;
    }

    return false;
}

pub fn display() {

}

pub fn input() {

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
    use crate::usecase;

    #[test]
    fn test_iswin(){
        let board: Vec<Vec<String>> = vec![
        vec!["⚪︎".to_string(), "⚪︎".to_string(), "⚪︎".to_string()],
        vec!["-".to_string(), "-".to_string(), "-".to_string()],
        vec!["-".to_string(), "-".to_string(), "-".to_string()],
    ];
        let got: bool = usecase::iswin(&board); 
        assert_eq!(true,got);
    }

    #[test]
    fn test_input() {

    }

    #[test]
    fn test_display(){

    }

}