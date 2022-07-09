mod idt;
mod pic;

pub(crate) fn init() {
    unsafe { idt::init_idt() };
    unsafe { pic::PICS.lock().initialize()}
}