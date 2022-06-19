use crate::test_cap_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operational::structs::configure::ConfigureRegister;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;


pub trait ICreateConfigure {
    fn new_configure(&self, operational_base: u64) -> CreateRegisterResult<ConfigureRegister>;
}


impl ICreateConfigure for CreateType {
    fn new_configure(&self, operational_base: u64) -> CreateRegisterResult<ConfigureRegister> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(operational_base) }
        }
    }
}


fn uncheck_transmute(operational_base: u64) -> CreateRegisterResult<ConfigureRegister> {
    let start_addr = operational_base + 0x38;
    let configure = transmute_from_u64::<ConfigureRegister>(start_addr);
    
    Ok(Volatile::Core(RegisterInfo::new(start_addr, configure)))
}


test_cap_register!(should_new_configure, uncheck_transmute);
