use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateRuntimeRegisterSpaceOffset {
    fn new_runtime_register_space_offset(&self, mmio_base_addr: u64) -> CreateRegisterResult<RuntimeRegisterSpaceOffset>;
}

impl ICreateRuntimeRegisterSpaceOffset for CreateType{
    fn new_runtime_register_space_offset(&self, mmio_base_addr: u64) -> CreateRegisterResult<RuntimeRegisterSpaceOffset> {
        match self {
            CreateType::UncheckTransmute => {uncheck_transmute(mmio_base_addr)}
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<RuntimeRegisterSpaceOffset>{
    Ok(transmute_register(mmio_base_addr + 0x18))
}

test_cap_register!(should_new_runtime_register_space_offset, uncheck_transmute);
