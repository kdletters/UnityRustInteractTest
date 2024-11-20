use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;

lazy_static! {
    static ref Nearby: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1)
    ];
}

#[repr(C)]
#[derive(Clone)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub num: i32,
    pub is_mine: bool,
    pub is_flag: bool,
    pub is_opened: bool,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Block {
        Block {
            x,
            y,
            num: 0,
            is_mine: false,
            is_flag: false,
            is_opened: false,
        }
    }
}

pub struct MinesweeperGame {
    blocks: Vec<Block>,
    width: i32,
    height: i32,
    mine_count: i32,
    flag_count: i32,
    block_count: i32,
    start_timestamp: OffsetDateTime,

    on_game_over: Option<extern "C" fn(bool, i32, i32)>,
    on_open_block: Option<extern "C" fn(i32, i32)>,
}

impl MinesweeperGame {
    pub fn new(width: i32, height: i32, mine_count: i32) -> MinesweeperGame {
        let mut game = MinesweeperGame {
            height,
            width,
            mine_count,
            blocks: Vec::new(),
            flag_count: 0,
            block_count: height * width,
            start_timestamp: OffsetDateTime::now_utc(),
            on_game_over: None,
            on_open_block: None,
        };

        // create blocks
        for y in 0..width {
            for x in 0..height {
                game.blocks.push(Block::new(x, y));
            }
        }

        // generate mines
        let mut mine_num = 0;
        while mine_num < mine_count {
            let index = rand::random::<u32>() as usize % game.blocks.len();
            let block = &mut game.blocks[index];
            if block.is_mine {
                continue;
            }
            block.is_mine = true;
            mine_num = mine_num + 1;
        }

        // update num
        for i in 0..game.blocks.len() {
            let mut num = 0;
            let cur_block = &game.blocks[i];
            for (dx, dy) in Nearby.iter() {
                let x = cur_block.x + dx;
                let y = cur_block.y + dy;
                if x >= 0 && x < width && y >= 0 && y < height {
                    if let Some(other_block) = game.get_block(x, y) {
                        if other_block.is_mine {
                            num += 1;
                        }
                    }
                }
            }

            game.blocks[i].num = num;
        }

        game
    }

    /// open block in (x, y)
    pub fn open_block(&mut self, x: i32, y: i32, by_click: bool) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }

        let index = (self.width * y + x) as usize;
        if let Some(block) = self.blocks.get_mut(index) {
            if block.is_opened || block.is_flag {
                return false;
            }

            block.is_opened = true;
            self.block_count -= 1;
            if let Some(callback) = self.on_open_block {
                callback(x, y);
            }
            if block.is_mine {
                // Game over
                if let Some(callback) = self.on_game_over {
                    callback(false, x, y);
                }

                return false;
            } else {
                // Open block
                if block.num == 0 {
                    // Open nearby blocks
                    Nearby.iter().for_each(|(x1, y1)| {
                        self.open_block(x + x1, y + y1, false);
                    });
                }

                if self.block_count == self.mine_count {
                    if let Some(callback) = self.on_game_over {
                        // Win
                        callback(true, x, y);
                    }
                }
                return true;
            }
        }

        return false;
    }

    pub fn quick_open(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }

        if let Some(block) = self.get_block(x, y) {
            if !block.is_opened || block.num == 0 {
                return;
            }

            let flag_num = Nearby.iter().fold(0, |sum, (dx, dy)| {
                sum + self
                    .get_block(x + dx, y + dy)
                    .map_or(0, |block| if block.is_flag { 1 } else { 0 })
            });

            if block.num == flag_num {
                for (nx, ny) in self.get_nearby(x, y).iter() {
                    self.open_block(*nx, *ny, true);
                }
            }
        }
    }

    pub fn get_block(&self, x: i32, y: i32) -> Option<&Block> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        let index = (self.width * y + x) as usize;
        self.blocks.get(index)
    }

    pub fn get_nearby(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        Nearby
            .iter()
            .map(|(dx, dy)| self.get_block(x + dx, y + dy))
            .skip_while(|o| o.is_none())
            .map(|o| {
                let block = o.unwrap();
                return (block.x, block.y);
            })
            .collect()
    }

    pub fn flag_block(&mut self, x: i32, y: i32) -> bool {
        let index = (self.width * y + x) as usize;
        if let Some(block) = self.blocks.get_mut(index) {
            if block.is_flag {
                self.flag_count -= 1;
            } else {
                self.flag_count += 1;
            }
            block.is_flag = !block.is_flag;

            return true;
        }

        return false;
    }
}

impl Display for MinesweeperGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "    ")?;
        for i in 0..self.width {
            write!(f, " {}  ", i)?;
        }
        write!(f, "\n")?;
        for x in &self.blocks {
            if x.x == 0 {
                write!(f, "{} ||", x.y)?;
            }
            if x.is_opened {
                if x.is_mine {
                    f.write_str("(x)")?;
                } else if x.is_flag {
                    f.write_str("(P)")?;
                } else {
                    write!(f, "({})", x.num)?;
                }
            } else {
                f.write_str("( )")?;
            }

            f.write_str("|")?;

            if x.x == self.width - 1 {
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn create_game(width: i32, height: i32, mine_count: i32) -> *mut MinesweeperGame {
    Box::into_raw(Box::new(MinesweeperGame::new(width, height, mine_count)))
}

#[no_mangle]
pub unsafe extern "C" fn free_game(game: *mut MinesweeperGame) {
    drop(Box::from_raw(game));
}

#[no_mangle]
pub unsafe extern "C" fn set_on_game_over(
    game: *mut MinesweeperGame,
    callback: Option<extern "C" fn(bool, i32, i32)>,
) {
    let game = &mut *game;
    game.on_game_over = callback;
}

#[no_mangle]
pub unsafe extern "C" fn set_on_open_block(
    game: *mut MinesweeperGame,
    callback: Option<extern "C" fn(i32, i32)>,
) {
    let game = &mut *game;
    game.on_open_block = callback;
}

#[no_mangle]
pub unsafe extern "C" fn open_block(game: *mut MinesweeperGame, x: i32, y: i32) -> bool {
    let game = &mut *game;
    game.open_block(x, y, true)
}

#[no_mangle]
pub unsafe extern "C" fn quick_open(game: *mut MinesweeperGame, x: i32, y: i32) {
    let game = &mut *game;
    game.quick_open(x, y);
}

#[no_mangle]
pub unsafe extern "C" fn flag_block(game: *mut MinesweeperGame, x: i32, y: i32) -> bool {
    let game = &mut *game;
    game.flag_block(x, y)
}

#[no_mangle]
pub unsafe extern "C" fn get_block(game: *mut MinesweeperGame, x: i32, y: i32) -> *mut Block {
    let game = &mut *game;
    if let Some(block) = game.get_block(x, y) {
        block as *const Block as *mut Block
    } else {
        panic!("Invalid block index")
    }
}