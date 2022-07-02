use crate::test_runtime_register;
use crate::usb_my::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb_my::xhci::registers::runtime::structs::interrupter::mf_index::MicroFrameIndex;
use crate::utils::raw_ptr::transmute_register;

pub trait ICreateMfIndex {
    fn new_mf_index(&self, runtime_base: u64) -> CreateRegisterResult<MicroFrameIndex>;
}

impl ICreateMfIndex for RegisterCreate {
    fn new_mf_index(&self, runtime_base: u64) -> CreateRegisterResult<MicroFrameIndex> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(runtime_base),
        }
    }
}

fn uncheck_transmute(runtime_base: u64) -> CreateRegisterResult<MicroFrameIndex> {
    Ok(transmute_register(runtime_base))
}

test_runtime_register!(should_new_imod, uncheck_transmute);