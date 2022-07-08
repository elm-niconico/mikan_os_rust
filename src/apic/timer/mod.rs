mod timer_manager;

// pub(crate) extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
//     log!(".");
//     APIC_TIMER.lock().tick();
//     notify_end_of_interrupt();
// }