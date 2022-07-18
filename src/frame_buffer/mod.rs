/* Qemuのコンソールに文字を出力するための機構を提供するクレートになります */

use core::fmt::{self};

use bootloader::boot_info::FrameBuffer;

use crate::frame_buffer::pixel::pixel_color::PixelColor;
use crate::spin::sync_mutex::StaticSpinMutex;
use crate::spin::sync_once_cell::StaticOnceCell;

use self::blog_os::BlogOsWriter;

pub mod blog_os;
pub mod pixel;

pub static WRITER: StaticOnceCell<StaticSpinMutex<BlogOsWriter>> = StaticOnceCell::uninit();


#[allow(unused)]
pub fn frame_buff_size() -> (usize, usize) {
    let frame_buff = WRITER.get().lock();
    (frame_buff.width(), frame_buff.height())
}

pub fn init(frame_buffer: &'static mut FrameBuffer) {
    let mut writer = BlogOsWriter::new(frame_buffer);
    writer.clear();

    WRITER.init_once(|| StaticSpinMutex::new(writer));
}


pub trait PixelWriter {
    fn write_pixel(&mut self, x: usize, y: usize, color: PixelColor);
    fn clear_pixel(&mut self, x: usize, y: usize);
}


#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub fn fill_rect(pixel_writer: &mut impl PixelWriter,
                 rect: Rect,
                 color: PixelColor,
) {
    for h in 0..rect.height {
        for w in 0..rect.width {
            pixel_writer.write_pixel(w + rect.x, h + rect.y, color);
        }
    }
}

pub fn draw_rect(pixel_writer: &mut impl PixelWriter,
                 rect: Rect,
                 color: PixelColor,
) {
    for w in 0..rect.width {
        // 上辺
        pixel_writer.write_pixel(rect.x + w, rect.y, color);
        // 底辺
        pixel_writer.write_pixel(rect.x + w, rect.y + rect.height - 1, color);
    }

    for h in 0..rect.height {
        // 左辺
        pixel_writer.write_pixel(rect.x, rect.y + h, color);
        // 右辺
        pixel_writer.write_pixel(rect.x + rect.width - 1, rect.y + h, color);
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.get().lock().write_fmt(args).unwrap();
    });
}