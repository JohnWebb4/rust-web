use core::sync::atomic::{AtomicU32, Ordering};

const HEIGHT: usize = 350;
const SPEED: u32 = 1;
const START_FRAME: u32 = 0;
const WIDTH: usize = 900;

static FRAME: AtomicU32 = AtomicU32::new(START_FRAME);

extern "C" {
    fn console_log(i: u8);
}

#[no_mangle]
static mut BUFFER: [u8; 4 * WIDTH * HEIGHT] = [0; 4 * WIDTH * HEIGHT];

#[no_mangle]
pub unsafe extern "C" fn run() {
    render_frame_safe(&mut BUFFER);
}

fn render_frame_safe(buffer: &mut [u8; 4 * WIDTH * HEIGHT]) {
    let f = FRAME.fetch_add(SPEED, Ordering::Relaxed);

    for y in 0..HEIGHT {
        for x in 0..4 * WIDTH {
            let pixel: u32 = ((x * y) | 0xFF_00_FF_00) as u32;

            buffer[4 * y * WIDTH + x] = pixel.wrapping_add(f) as u8;
        }
    }
}
