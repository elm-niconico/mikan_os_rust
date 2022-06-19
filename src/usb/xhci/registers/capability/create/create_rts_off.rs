use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::rts_off::RuntimeRegisterSpaceOffset;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateRtsOff {
    /** New RuntimeRegisterSpaceOffset */
    fn new_rts_off(&self, mmio_base_addr: u64) -> CreateRegisterResult<RuntimeRegisterSpaceOffset>;
}


impl ICreateRtsOff for CreateType {
    fn new_rts_off(&self, mmio_base_addr: u64) -> CreateRegisterResult<RuntimeRegisterSpaceOffset> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(mmio_base_addr) }
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<RuntimeRegisterSpaceOffset> {
    Ok(transmute_register(mmio_base_addr + 0x18))
}

test_cap_register!(should_new_rts_off, uncheck_transmute);
