use x86_64::VirtAddr;

use crate::{FRAME_ALLOCATOR, log, PAGE_TABLE, serial_println};
use crate::memory::paging::make_identity_mapping;

mod idt;
//mod pic;
pub mod apic;


pub(crate) fn init(phys_offset: VirtAddr) {
    unsafe { idt::init_idt() };
    serial_println!("Init IDT");

    //unsafe { pic::PICS.lock().initialize() }

    const TIMER_FRAME_BASE_ADDR: u64 = 0xfee00000;
    map(TIMER_FRAME_BASE_ADDR);

    apic::timer::timer_manager::APIC_TIMER.lock().init();
    log!("Init APIC Timer");
   
    apic::mouse::init(phys_offset);
    serial_println!("Init Xhc Mouse Controller");
}

pub fn map(rsdp: u64) {
    let mapper = &mut *(unsafe { PAGE_TABLE.get_unchecked() }.lock());
    let frame_allocator = &mut *FRAME_ALLOCATOR.lock();
    let base_addr = VirtAddr::new(rsdp).align_down(4096u64).as_u64();
    make_identity_mapping(mapper, frame_allocator, base_addr, 1).expect("Failed Rsdp Mapping");
}