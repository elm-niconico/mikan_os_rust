use crate::test_op_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::operational::structs::dcbaap::Dcbaap;
use crate::utils::raw_ptr::transmute_register;

/** DeviceContextBaseAddressArrayPointer */
pub trait ICreateDcbaap {
    /** DeviceContextBaseAddressArrayPointer */
    fn new_dcbaap(&self, operational_base_addr: u64) -> CreateRegisterResult<Dcbaap>;
}

impl ICreateDcbaap for RegisterCreate {
    fn new_dcbaap(&self, operational_base_addr: u64) -> CreateRegisterResult<Dcbaap> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(operational_base_addr),
            _ => {
                todo!()
            }
        }
    }
}

fn uncheck_transmute(operational_base_addr: u64) -> CreateRegisterResult<Dcbaap> {
    let addr = operational_base_addr + 0x30;
    Ok(transmute_register(addr))
}

test_op_register!(should_new_dcbap, uncheck_transmute);