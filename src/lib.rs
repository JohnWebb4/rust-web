use std::mem;
use std::slice;
use std::os::raw::c_void;

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
    let byte_size = max_width * max_height * 4;
    let s1 = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    for i in 0..byte_size {
        let height = i / 4 / max_width;
        let width = i / 4 % max_width;

        if i % 4 == 3 {
            s1[i] = 255;
        } else if i % 4 == 0 {
            // Purple circle
            let len = ((height * height + width * width) as f64).sqrt();
            let nb = time + len / 4.0;
            let a = 128.0 + nb.cos() * 128.0;
            s1[i] = a as u8;
        } else if i % 4 == 2 {
            // Blue circle
            let width = max_width - width;
            let len = ((height*height + width*width) as f64).sqrt();
            let nb = time + len / 4.0;
            let a = 128.0 + nb.cos() * 128.0;
            s1[i] = a as u8;
        }
    }

}

