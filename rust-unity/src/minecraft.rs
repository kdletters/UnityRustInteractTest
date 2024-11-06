use std::ffi::c_void;
use std::fmt::Pointer;
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

pub struct MinecraftGame {
    blocks: Vec<Block>,
    width: i32,
    height: i32,
    mine_count: i32,
    flag_count: i32,
    start_timestamp: OffsetDateTime,
}

impl MinecraftGame {
    pub fn new(width: i32, height: i32, mine_count: i32) -> MinecraftGame {
        let mut game = MinecraftGame {
            height,
            width,
            mine_count,
            blocks: Vec::new(),
            flag_count: 0,
            start_timestamp: OffsetDateTime::now_utc(),
        };

        for x in 0..width {
            for y in 0..height {
                game.blocks.push(Block::new(x, y));
            }
        }

        game
    }

    pub fn open_block(&mut self, x: i32, y: i32) {
        let index = (self.width * y + x) as usize;
        if let Some(block) = self.blocks.get_mut(index) {
            if block.is_mine {
                // Game over
            } else {
                // Open block
                if !block.is_opened {
                    block.is_opened = true;
                }
            }
        }
    }

    pub fn get_block(&mut self, x: i32, y: i32) -> *mut Block {
        let index = (self.width * y + x) as usize;
        if let Some(block) = self.blocks.get_mut(index) {
            block as *mut Block
        } else {
            panic!("Invalid block index")
        }
    }
}

#[no_mangle]
pub extern "C" fn create_game(width: i32, height: i32, mine_count: i32) -> *mut c_void {
    Box::into_raw(Box::new(MinecraftGame::new(width, height, mine_count))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn free_game(game: *mut c_void) {
    drop(Box::from_raw(game));
}

#[no_mangle]
pub unsafe extern "C" fn open_block(game: *mut c_void, x: i32, y: i32) {
    let mut game = Box::from_raw(game as *mut MinecraftGame);
    game.open_block(x, y);
    let _ = Box::into_raw(game);
}

#[no_mangle]
pub unsafe extern "C" fn get_block(game: *mut c_void, x: i32, y: i32) -> *mut Block {
    let mut game = Box::from_raw(game as *mut MinecraftGame);
    let block = game.get_block(x, y);
    let _ = Box::into_raw(game);
    block
}