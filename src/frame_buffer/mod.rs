/* Qemuのコンソールに文字を出力するための機構を提供するクレートになります */

use core::fmt::{self, Write};

use bootloader::boot_info::FrameBuffer;
use spin::mutex::Mutex;

use self::blog_os::BlogOsWriter;

mod blog_os;

//FIXME サイズがわからないため、現在は抽象化できていない
pub static WRITER: Mutex<Option<BlogOsWriter>> = Mutex::new(None);

pub fn init(frame_buffer: &'static mut FrameBuffer) {
    let mut writer = BlogOsWriter::new(frame_buffer);
    writer.clear();

    // global writer should not be locked here
    let mut global_writer = WRITER.try_lock().unwrap();
    assert!(global_writer.is_none(), "Global writer already initialized");
    *global_writer = Some(writer);
}

/// Prints the given formatted string to the VGA text buffer
/// through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().as_mut().unwrap().write_fmt(args).unwrap();
    });
}