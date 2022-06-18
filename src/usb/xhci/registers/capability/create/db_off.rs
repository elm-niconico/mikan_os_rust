use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::db_off::DoorbellOffsetRegister;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateDbOff {
    fn new_db_off(&self, mmio_base_addr: u64) -> CreateRegisterResult<DoorbellOffsetRegister>;
}

impl ICreateDbOff for CreateType{
    fn new_db_off(&self, mmio_base_addr: u64) -> CreateRegisterResult<DoorbellOffsetRegister> {
        match self {
            CreateType::UncheckTransmute => {uncheck_transmute(mmio_base_addr)}
        }
    }
}

fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<DoorbellOffsetRegister>{
    Ok(transmute_register(mmio_base_addr + 0x14))
}

test_cap_register!(should_new_db_off, uncheck_transmute);

