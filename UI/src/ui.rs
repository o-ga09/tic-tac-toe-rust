use std::io::{Read};

pub struct GameUi {}

impl GameUi {
    pub fn input(&self,input: &mut dyn Read) -> String {
        let mut str = String::new();
        let _ = input.read_to_string(&mut str);
        return str.to_string();
    }
    
    pub fn output(&self,output: &Vec<Vec<String>>) -> Result<(),&'static str> {
        for i in 0..3 {
            for j in 0..3 {
                print!("{}",output[i][j]);
            }
            println!()
        }
        Ok(())
    }
}

// テストモジュール
#[cfg(test)]

mod tests{
    use std::io::Cursor;
    use crate::ui::{GameUi};

    #[test]
    fn test_input() {
        let input_str = "want";
        let mut input_buffer = Cursor::new(input_str);
        let game_driver: GameUi = GameUi {};
        let result = game_driver.input(&mut input_buffer);

        assert_eq!(result, "want".to_string());
    }

    #[test]
    fn test_output() {
        let arg = vec![
            vec!["X".to_string(), "O".to_string(), "X".to_string()],
            vec!["O".to_string(), "X".to_string(), "O".to_string()],
            vec!["X".to_string(), "O".to_string(), "X".to_string()],
        ];
        let game_driver: GameUi = GameUi {};
        let got =game_driver.output(&arg);
        assert_eq!(got, Ok(()));
    }
}
