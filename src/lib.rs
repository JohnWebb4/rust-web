use core::sync::atomic::{AtomicU32, Ordering};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const START_FRAME: u32 = 255;

static FRAME: AtomicU32 = AtomicU32::new(START_FRAME);

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[no_mangle]
pub unsafe extern fn run() {
    render_frame_safe(&mut BUFFER)
}

fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel: u32 = ((x * y) | 0xFF_00_FF_00) as u32;

            buffer[y * WIDTH + x] = f.wrapping_add(pixel);
        }
    }
}