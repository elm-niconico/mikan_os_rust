use crate::test_runtime_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::runtime::structs::interrupter::imod::InterrupterModerationRegister;
use crate::utils::raw_ptr::transmute_register;

pub trait ICreateIMod {
    fn new_imod(&self, runtime_base: u64) -> CreateRegisterResult<InterrupterModerationRegister>;
}

impl ICreateIMod for RegisterCreate {
    fn new_imod(&self, runtime_base: u64) -> CreateRegisterResult<InterrupterModerationRegister> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(runtime_base),
        }
    }
}

fn uncheck_transmute(runtime_base: u64) -> CreateRegisterResult<InterrupterModerationRegister> {
    Ok(transmute_register(runtime_base + 0x24))
}

test_runtime_register!(should_new_imod, uncheck_transmute);
