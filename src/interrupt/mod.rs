
use crate::{log, serial_println};

mod idt;
pub mod pic;
pub mod apic;


pub(crate) fn init() {
    unsafe { idt::init_idt() };
    serial_println!("Init IDT");

    unsafe { pic::PICS.lock().initialize() }

    // const TIMER_FRAME_BASE_ADDR: u64 = 0xfee00000;
    // map(TIMER_FRAME_BASE_ADDR);

    //
    // apic::timer::timer_manager::APIC_TIMER.lock().init();
    // log!("Init APIC Timer");

    // unsafe { apic::mouse::init(phys_offset); }
    // serial_println!("Init Xhc Mouse Controller");
}