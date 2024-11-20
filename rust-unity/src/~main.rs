use rust_unity::minesweeper::MinesweeperGame;
use std::io::{BufRead, Write};
use std::str::FromStr;

fn main() {
    let mut game = MinesweeperGame::new(10, 10, 10);
    let mut buf = String::default();
    print!("{}", game);
    loop {
        let _ = std::io::stdout().flush();
        let mut reader = std::io::stdin().lock();
        buf.clear();
        let _ = reader.read_line(&mut buf);
        
        let splits = buf.trim().split(" ").collect::<Vec<_>>();
        let x = if splits.len() == 2 {
            i32::from_str(splits[0]).unwrap()
        } else {
            rand::random::<i32>() % 10
        };
        let y = if splits.len() == 2 {
            i32::from_str(splits[1]).unwrap()
        } else {
            rand::random::<i32>() % 10
        };
        
        game.open_block(x, y, true);
        print!("{}", game);
    }
}
