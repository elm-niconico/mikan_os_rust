use pc_keyboard::KeyCode::P;
use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupt::pic::PICS;
use crate::{log, print};

pub extern "x86-interrupt" fn pic_timer_handler(stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe { PICS.lock().notify_end_of_interrupt(0x20); }
}