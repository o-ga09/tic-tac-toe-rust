mod tictactoe;

fn main() {
    let mut input = std::io::stdin();
    let res:String = tictactoe::input(&mut input);
    println!("{}",res);
}
