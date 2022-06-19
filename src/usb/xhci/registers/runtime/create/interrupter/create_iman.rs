use crate::test_runtime_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::runtime::structs::interrupter::iman::InterrupterManagementRegister;
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateIman {
    fn new_iman(&self, runtime_base: u64) -> CreateRegisterResult<InterrupterManagementRegister>;
}


impl ICreateIman for CreateType {
    fn new_iman(&self, runtime_base: u64) -> CreateRegisterResult<InterrupterManagementRegister> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(runtime_base) }
        }
    }
}


fn uncheck_transmute(runtime_base: u64) -> CreateRegisterResult<InterrupterManagementRegister> {
    Ok(transmute_register(runtime_base))
}


test_runtime_register!(should_new_iman, uncheck_transmute);
