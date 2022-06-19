use crate::test_op_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operational::structs::dcbaap::Dcbaap;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;


/** DeviceContextBaseAddressArrayPointer */
pub trait ICreateDcbaap {
    /** DeviceContextBaseAddressArrayPointer */
    fn new_dcbaap(&self, operational_base_addr: u64) -> CreateRegisterResult<Dcbaap>;
}


impl ICreateDcbaap for CreateType {
    fn new_dcbaap(&self, operational_base_addr: u64) -> CreateRegisterResult<Dcbaap> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(operational_base_addr) }
            _ => { todo!() }
        }
    }
}


fn uncheck_transmute(operational_base_addr: u64) -> CreateRegisterResult<Dcbaap> {
    let addr = operational_base_addr + 0x30;
    
    let dcbaap = transmute_from_u64::<>(addr);
    
    Ok(Volatile::Core(RegisterInfo::new(addr, dcbaap)))
}

test_op_register!(should_new_dcbap, uncheck_transmute);
