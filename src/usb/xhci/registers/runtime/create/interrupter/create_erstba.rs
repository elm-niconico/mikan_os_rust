use crate::test_runtime_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::runtime::structs::interrupter::erstba::EventRingSegmentTableBaseAddressRegister;
use crate::utils::raw_ptr::transmute_register;

pub trait CreateErstba {
    fn new_erstba(
        &self,
        runtime_base: u64,
    ) -> CreateRegisterResult<EventRingSegmentTableBaseAddressRegister>;
}

impl CreateErstba for RegisterCreate {
    fn new_erstba(
        &self,
        runtime_base: u64,
    ) -> CreateRegisterResult<EventRingSegmentTableBaseAddressRegister> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(runtime_base),
        }
    }
}

fn uncheck_transmute(
    runtime_base: u64,
) -> CreateRegisterResult<EventRingSegmentTableBaseAddressRegister> {
    Ok(transmute_register(runtime_base + 0x30))
}

test_runtime_register!(should_new_erstza, uncheck_transmute);