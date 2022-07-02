use crate::test_cap_register;
use crate::usb_my::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb_my::xhci::registers::operational::structs::configure::ConfigureRegister;
use crate::usb_my::xhci::registers::register_info::RegisterInfo;
use crate::usb_my::xhci::registers::volatile::Volatile;
use crate::utils::raw_ptr::transmute_register;

pub trait ICreateConfigure {
    fn new_configure(&self, operational_base: u64) -> CreateRegisterResult<ConfigureRegister>;
}

impl ICreateConfigure for RegisterCreate {
    fn new_configure(&self, operational_base: u64) -> CreateRegisterResult<ConfigureRegister> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(operational_base),
        }
    }
}

fn uncheck_transmute(operational_base: u64) -> CreateRegisterResult<ConfigureRegister> {
    let config_addr = operational_base + 0x38;
    Ok(transmute_register(config_addr))
}

test_cap_register!(should_new_configure, uncheck_transmute);