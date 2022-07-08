mod idt;

pub(crate) fn init() {
    unsafe { idt::init_idt() };
}