use crate::test_op_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operational::structs::command_ring_control::CommandRingControlRegister;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;


pub trait ICreateCrcr {
    /** New Command Ring Control Register */
    fn new_crcr(&self, operational_base_addr: u64) -> CreateRegisterResult<CommandRingControlRegister>;
}


impl ICreateCrcr for CreateType {
    fn new_crcr(&self, operational_base_addr: u64) -> CreateRegisterResult<CommandRingControlRegister> {
        match self {
            CreateType::UncheckTransmute => {
                uncheck_transmute(operational_base_addr)
            }
        }
    }
}


fn uncheck_transmute(operational_base_addr: u64) -> CreateRegisterResult<CommandRingControlRegister> {
    let addr = operational_base_addr + 0x18;
    
    let command_ring_control = transmute_from_u64::<CommandRingControlRegister>(addr);
    Ok(Volatile::Core(RegisterInfo::new(addr, command_ring_control)))
}


test_op_register!(should_new_crcr, uncheck_transmute);
