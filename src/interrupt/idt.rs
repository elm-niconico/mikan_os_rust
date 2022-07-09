use pc_keyboard::KeyCode::P;
use volatile::Volatile;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::interrupt::apic::timer::apic_timer_handler;
use crate::interrupt::pic;
use crate::log;
use crate::spin::sync_once_cell::StaticOnceCell;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    PicTimer = 32,
    APicTimer = 0x41,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
    pub fn as_u32(self) -> u32 {
        self as u32
    }
    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub(crate) static mut IDT: StaticOnceCell<InterruptDescriptorTable> = StaticOnceCell::uninit();

pub(crate) unsafe fn init_idt() {
    IDT.init_once(|| create_idt());
    IDT.get().load();
}

pub(crate) fn create_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();

    idt
        .breakpoint
        .set_handler_fn(breakpoint_handler);

    idt
        .segment_not_present
        .set_handler_fn(segment_not_present_handler);

    idt
        .general_protection_fault
        .set_handler_fn(general_protection_fault_handler);

    idt
        .page_fault
        .set_handler_fn(page_fault_handler);

    idt
        .invalid_tss
        .set_handler_fn(invalid_tss_handler);


    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(0);
    }

    idt[InterruptIndex::APicTimer.as_usize()].set_handler_fn(apic_timer_handler);
    idt
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!(
        "EXCEPTION: PROTECTION ERROR CODE {} \n{:#?}",
        error_code, stack_frame
    );
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log!("Interrupt Breakpoint Handler {:?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    log!("EXCEPTION: PAGE FAULT");
    log!("Accessed Address: {:?}", Cr2::read());
    log!("Error Code: {:?}", error_code);
    log!("{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT ERROR CODE: {} \n{:#?}",
        _error_code, stack_frame
    );
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!(
        "EXCEPTION: SEGMENT NOT PRESENT ERROR CODE: {} \n{:#?}",
        _error_code, stack_frame
    );
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!(
        "EXCEPTION: INVALID TSS ERROR CODE: {} \n{:#?}",
        _error_code, stack_frame
    );
}