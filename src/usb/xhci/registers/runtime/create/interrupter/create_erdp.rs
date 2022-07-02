use crate::test_runtime_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::runtime::structs::interrupter::erdp::EventRingDequePointerRegister;
use crate::utils::raw_ptr::transmute_register;

pub trait CreateErdp {
    fn new_erdp(&self, runtime_base: u64) -> CreateRegisterResult<EventRingDequePointerRegister>;
}

impl CreateErdp for RegisterCreate {
    fn new_erdp(&self, runtime_base: u64) -> CreateRegisterResult<EventRingDequePointerRegister> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(runtime_base),
        }
    }
}

fn uncheck_transmute(runtime_base: u64) -> CreateRegisterResult<EventRingDequePointerRegister> {
    Ok(transmute_register(runtime_base + 0x38))
}

test_runtime_register!(should_new_runtime, uncheck_transmute);