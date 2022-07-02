use crate::test_runtime_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::runtime::structs::interrupter::erstsz::EventRingSegmentTableSizeRegister;
use crate::utils::raw_ptr::transmute_register;

pub trait CreateErstsz {
    fn new_erstsz(
        &self,
        runtime_base: u64,
    ) -> CreateRegisterResult<EventRingSegmentTableSizeRegister>;
}

impl CreateErstsz for RegisterCreate {
    fn new_erstsz(
        &self,
        runtime_base: u64,
    ) -> CreateRegisterResult<EventRingSegmentTableSizeRegister> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(runtime_base),
        }
    }
}

fn uncheck_transmute(runtime_base: u64) -> CreateRegisterResult<EventRingSegmentTableSizeRegister> {
    Ok(transmute_register(runtime_base + 0x28))
}

test_runtime_register!(should_new_erstsz, uncheck_transmute);