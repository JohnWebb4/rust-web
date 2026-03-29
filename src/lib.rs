use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::prelude::*;

const HEIGHT: usize = 350;
const SPEED: u32 = 1;
const START_FRAME: u32 = 0;
const WIDTH: usize = 900;

static FRAME: AtomicU32 = AtomicU32::new(START_FRAME);

struct SyncBuffer(UnsafeCell<[u8; 4 * WIDTH * HEIGHT]>);
unsafe impl Sync for SyncBuffer {}

static BUFFER: SyncBuffer = SyncBuffer(UnsafeCell::new([0; 4 * WIDTH * HEIGHT]));

#[wasm_bindgen]
pub fn buffer_ptr() -> *mut u8 {
    BUFFER.0.get() as *mut u8
}

#[wasm_bindgen]
pub fn run() {
    let f = FRAME.fetch_add(SPEED, Ordering::Relaxed);
    let buffer = unsafe { &mut *BUFFER.0.get() };

    for y in 0..HEIGHT {
        for x in 0..4 * WIDTH {
            let pixel: u32 = ((x * y) | 0xFF_00_FF_00) as u32;
            buffer[4 * y * WIDTH + x] = pixel.wrapping_add(f) as u8;
        }
    }
}
