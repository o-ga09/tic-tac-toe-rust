// pub fn tictactoe() {

// }

use std::io::{Read};

pub fn input(input: &mut dyn Read) -> String {
    let mut str = String::new();
    let _ = input.read_to_string(&mut str);
    return "mwed".to_string();
}

// テストモジュール
#[cfg(test)]

mod tests{
    use std::io::{Read};
    use crate::tictactoe;

    #[test]
    fn test_input() {
        let mut stdin = std::io::stdin();

        // 標準入力に "Hello, world!" を書き込む
        let  s: &mut String = &mut "abc".to_string();
        stdin.read_to_string(s).unwrap();
    
        // input 関数を呼び出す
        let str = tictactoe::input(&mut stdin);
    
        // 期待される値と一致することを確認する
        assert_eq!(str, "Hello, world!");
    }
}