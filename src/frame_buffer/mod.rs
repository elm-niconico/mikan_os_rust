/* Qemuのコンソールに文字を出力するための機構を提供するクレートになります */

use core::fmt::{self};

use bootloader::boot_info::FrameBuffer;

use crate::spin::sync_mutex::StaticSpinMutex;
use crate::spin::sync_once_cell::StaticOnceCell;

use self::blog_os::BlogOsWriter;

mod blog_os;

pub static WRITER: StaticOnceCell<StaticSpinMutex<BlogOsWriter>> = StaticOnceCell::uninit();

pub fn frame_buff_size() -> (usize, usize) {
    let frame_buff = WRITER.get().lock();
    return (frame_buff.width(), frame_buff.height());
}

pub fn init(frame_buffer: &'static mut FrameBuffer) {
    let mut writer = BlogOsWriter::new(frame_buffer);
    writer.clear();

    writer.clear();
    WRITER.init_once(|| StaticSpinMutex::new(writer));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.get().lock().write_fmt(args).unwrap();
    });
}