use volatile::Volatile;
use x86_64::structures::idt::InterruptStackFrame;

use crate::print;

pub mod timer_manager;


pub fn notify_end_of_interrupt() {
    // アドレスはなんでもいいらしい
    let notify_addr: u32 = 0xfee000b0;
    let mut memory = Volatile::new(unsafe { (notify_addr as *mut u32).as_mut().unwrap() });
    memory.write(0);
}

pub extern "x86-interrupt" fn apic_timer_handler(_: InterruptStackFrame) {
    print!(".");
    let mut memory = Volatile::new(unsafe { (0xfee000b0 as *mut u32).as_mut().unwrap() });
    memory.write(0);
    // notify_end_of_interrupt();
}