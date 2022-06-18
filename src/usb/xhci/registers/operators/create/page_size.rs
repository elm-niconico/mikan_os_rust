use crate::serial_println;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operators::structs::page_size::PageSizeRegister;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;
use crate::utils::test_fn::extract_operational_base_addr;


pub trait ICreatePageSize {
    fn new_page_size(&self, operational_base_addr: u64) -> CreateRegisterResult<PageSizeRegister>;
}


impl ICreatePageSize for CreateType {
    fn new_page_size(&self, operational_base_addr: u64) -> CreateRegisterResult<PageSizeRegister> {
        match self {
            CreateType::UncheckTransmute => { transmute_with_status_check(operational_base_addr) }
            // TODO StatusRegisterのチェック処理 アドレスを確認?
            CreateType::TransmuteWithCheck => { transmute_with_status_check(operational_base_addr) }
        }
    }
}


fn transmute_with_status_check(operational_base_addr: u64) -> CreateRegisterResult<PageSizeRegister> {
    // TODO StatusRegisterのチェック処理 アドレスを確認?
    let addr = operational_base_addr + 0x08;
    let page_size = transmute_from_u64::<PageSizeRegister>(addr);
    
    Ok(RegisterInfo::new(addr, page_size))
}


#[test_case]
pub fn should_transmute_page_size_register() {
    let page_size = transmute_with_status_check(extract_operational_base_addr());
    
    assert!(page_size.is_ok());
    serial_println!("{:?}", page_size.unwrap());
}



