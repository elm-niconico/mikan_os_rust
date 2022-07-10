/* Qemuのコンソールに文字を出力するための機構を提供するクレートになります */

use core::fmt::{self};

use bootloader::boot_info::FrameBuffer;
use spin::Mutex;

use self::blog_os::BlogOsWriter;

mod blog_os;


pub static WRITER: Mutex<Option<BlogOsWriter>> = Mutex::new(None);

pub fn init(frame_buffer: &'static mut FrameBuffer) {
    let mut writer = BlogOsWriter::new(frame_buffer);
    writer.clear();

    writer.clear();

    // global writer should not be locked here
    let mut global_writer = WRITER.try_lock().unwrap();
    assert!(global_writer.is_none(), "Global writer already initialized");
    *global_writer = Some(writer);
}


#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().as_mut().unwrap().write_fmt(args).unwrap();
    });
}