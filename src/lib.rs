use std::mem;
use std::slice;
use std::os::raw::c_void;

fn main() {}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buff = Vec::with_capacity(size);
    let ptr = buff.as_mut_ptr();
    mem::forget(buff);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn fill(pointer: *mut u8, max_width: usize, max_height: usize, time: f64) {

}

