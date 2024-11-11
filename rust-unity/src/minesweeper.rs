use time::OffsetDateTime;

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

        for y in 0..width {
            for x in 0..height {
                game.blocks.push(Block::new(x, y));
            }
        }

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

        for i in 0..game.blocks.len() {
            let mut num = 0;
            let mut block = &game.blocks[i];
            for (dx, dy) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                let x = block.x + dx;
                let y = block.y + dy;
                if x >= 0 && x < width && y >= 0 && y < height {
                    let index = (width * y + x) as usize;
                    if let Some(block) = game.blocks.get(index) {
                        if block.is_mine {
                            num += 1;;
                        }
                    }
                }
            }

            game.blocks[i].num = num;
        }

        game
    }

    pub fn open_block(&mut self, x: i32, y: i32) -> bool {
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
            if block.is_mine {
                // Game over
                if let Some(callback) = self.on_game_over {
                    callback(false, x, y);
                }

                return false;
            } else {
                // Open block
                if let Some(callback) = self.on_open_block {
                    callback(x, y);
                }

                if block.num == 0 {
                    self.open_block(x - 1, y);
                    self.open_block(x + 1, y);
                    self.open_block(x - 1, y - 1);
                    self.open_block(x, y - 1);
                    self.open_block(x + 1, y - 1);
                    self.open_block(x - 1, y + 1);
                    self.open_block(x, y + 1);
                    self.open_block(x + 1, y + 1);
                }

                if self.block_count == self.mine_count {
                    if let Some(callback) = self.on_game_over {
                        callback(true, x, y);
                    }
                }
                return true;
            }
        }

        return false;
    }

    pub fn get_block(&mut self, x: i32, y: i32) -> *mut Block {
        let index = (self.width * y + x) as usize;
        if let Some(block) = self.blocks.get_mut(index) {
            block as *mut Block
        } else {
            panic!("Invalid block index")
        }
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

#[no_mangle]
pub extern "C" fn create_game(width: i32, height: i32, mine_count: i32) -> *mut MinesweeperGame {
    Box::into_raw(Box::new(MinesweeperGame::new(width, height, mine_count)))
}

#[no_mangle]
pub unsafe extern "C" fn free_game(game: *mut MinesweeperGame) {
    drop(Box::from_raw(game));
}

#[no_mangle]
pub unsafe extern "C" fn set_on_game_over(game: *mut MinesweeperGame, callback: Option<extern "C" fn(bool, i32, i32)>) {
    let game = &mut *game;
    game.on_game_over = callback;
}

#[no_mangle]
pub unsafe extern "C" fn set_on_open_block(game: *mut MinesweeperGame, callback: Option<extern "C" fn(i32, i32)>) {
    let game = &mut *game;
    game.on_open_block = callback;
}

#[no_mangle]
pub unsafe extern "C" fn open_block(game: *mut MinesweeperGame, x: i32, y: i32) -> bool {
    let game = &mut *game;
    game.open_block(x, y)
}

#[no_mangle]
pub unsafe extern "C" fn flag_block(game: *mut MinesweeperGame, x: i32, y: i32) -> bool {
    let game = &mut *game;
    game.flag_block(x, y)
}

#[no_mangle]
pub unsafe extern "C" fn get_block(game: *mut MinesweeperGame, x: i32, y: i32) -> *mut Block {
    let game = &mut *game;
    game.get_block(x, y)
}