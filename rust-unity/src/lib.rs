use std::ffi::c_void;
use core::ByteBuffer;

mod minesweeper;
mod core;

#[repr(C)]
pub struct Obj {
    pub name: *const ByteBuffer,
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }

#[no_mangle]
pub extern "C" fn get() -> *const Obj {
    Box::into_raw(Box::new(Obj { name: Box::into_raw(Box::new(ByteBuffer::from_vec("我是结构体里的傻逼".to_string().into_bytes()))) }))
}

#[no_mangle]
pub extern "C" fn alloc_u8_string() -> *mut ByteBuffer {
    let str = format!("foo bar baz");
    let buf = ByteBuffer::from_vec(str.into_bytes());
    Box::into_raw(Box::new(buf))
}

#[no_mangle]
pub extern "C" fn alloc_u8_buffer() -> *mut ByteBuffer {
    let vec: Vec<u8> = vec![1, 10, 100];
    let buf = ByteBuffer::from_vec(vec);
    Box::into_raw(Box::new(buf))
}

#[no_mangle]
pub extern "C" fn alloc_i32_buffer() -> *mut ByteBuffer {
    let vec: Vec<i32> = vec![1, 10, 100, 1000, 10000];
    let buf = ByteBuffer::from_vec_struct(vec);
    Box::into_raw(Box::new(buf))
}

#[no_mangle]
pub unsafe extern "C" fn free_byte_buffer(buffer: *mut ByteBuffer) {
    let buf = Box::from_raw(buffer);
    // drop inner buffer, if you need Vec<i32>, use buf.destroy_into_vec_struct::<i32>() instead.
    buf.destroy();
}