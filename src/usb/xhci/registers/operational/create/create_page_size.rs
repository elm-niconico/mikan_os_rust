use crate::test_op_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::operational::structs::page_size::PageSizeRegister;
use crate::usb::xhci::registers::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;


pub trait ICreatePageSize {
    fn new_page_size(&self, operational_base_addr: u64) -> CreateRegisterResult<PageSizeRegister>;
}


impl ICreatePageSize for RegisterCreate {
    fn new_page_size(&self, operational_base_addr: u64) -> CreateRegisterResult<PageSizeRegister> {
        match self {
            RegisterCreate::UncheckTransmute => { uncheck_transmute(operational_base_addr) }
        }
    }
}


fn uncheck_transmute(operational_base_addr: u64) -> CreateRegisterResult<PageSizeRegister> {
    // TODO StatusRegisterのチェック処理 アドレスを確認?
    let addr = operational_base_addr + 0x08;
    let page_size = transmute_from_u64::<PageSizeRegister>(addr);
    
    Ok(Volatile::Core(RegisterInfo::new(addr, page_size)))
}


test_op_register!(should_new_page_size, uncheck_transmute);




