use lazy_static::lazy_static;
use pic8259::ChainedPics;
use volatile::Volatile;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::log_e;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    APicTimer = 0x41,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}


pub static mut PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(0);
        }

        idt[InterruptIndex::APicTimer.as_usize()].set_handler_fn(timer_interrupt_handler);

        log_e!("{:?}", idt[32]);
       // idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    //  let idt = InterruptDescriptorTable::new();
    // idt.general_protection_fault.se
    IDT.load();
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: PROTECTION ERROR CODE {} \n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log_e!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    log_e!("EXCEPTION: PAGE FAULT");
    log_e!("Accessed Address: {:?}", Cr2::read());
    log_e!("Error Code: {:?}", error_code);
    log_e!("{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT ERROR CODE: {} \n{:#?}", _error_code, stack_frame);
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!("EXCEPTION: SEGMENT NOT PRESENT ERROR CODE: {} \n{:#?}", _error_code, stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!("EXCEPTION: INVALID TSS ERROR CODE: {} \n{:#?}", _error_code, stack_frame);
}


extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    log_e!("timer!!");
    unsafe {
        let mut memory = Volatile::new(unsafe { (0xfee000b0 as *mut u32).as_mut().unwrap() });
        memory.write(0);
        //PICS.lock().notify_end_of_interrupt(Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    // let mut port = Port::new(0x60);
    // let scancode: u8 = unsafe { port.read() };
    // crate::task::keyboard::add_scancode(scancode);
    //
    // unsafe {
    //     // PICS.lock().notify_end_of_interrupt(Timer.as_u8());
    // }
}

// #[test_case]
// fn test_breakpoint_exception() {
//     // invoke a breakpoint exception
//     x86_64::instructions::interrupts::int3();
// }