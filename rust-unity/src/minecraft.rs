use crate::ByteBuffer;

#[repr(C)]
pub struct Mine {
    pub x : i32,
    pub y : i32,
    pub num : i32,
    pub is_mine : bool,
    pub is_flag : bool,
    pub is_opened : bool,
}

impl Mine {
    pub fn new(x : i32, y : i32) -> Mine {
        Mine {
            x,
            y,
            num : 0,
            is_mine : false,
            is_flag : false,
            is_opened : false,
        }
    }
}

#[no_mangle]
pub extern "C" fn create_mine(x : i32, y : i32) -> *mut Mine {
    Box::into_raw(Box::new(Mine::new(x, y)))
}

#[no_mangle]
pub extern "C" fn create_mines(width : i32, height : i32) -> *mut ByteBuffer {
    let mut vec = Vec::new();
    for x in 0..width {
        for y in 0..height {
            vec.push(Mine::new(x, y));
        }
    }

    Box::into_raw(Box::new(ByteBuffer::from_vec_struct::<Mine>(vec)))
}