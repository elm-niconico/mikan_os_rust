use crate::serial_println;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operational::structs::configure::ConfigureRegister;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;
use crate::utils::test_fn::extract_operational_base_addr;


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


#[test_case]
pub fn should_new_configure() {
    let configure = uncheck_transmute(extract_operational_base_addr());
    
    assert!(configure.is_ok());
    serial_println!("{:?}", configure.unwrap());
}
