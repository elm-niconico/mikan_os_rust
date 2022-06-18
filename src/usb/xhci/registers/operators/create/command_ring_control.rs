use crate::serial_println;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operators::structs::command_ring_control::CommandRingControlRegister;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;
use crate::utils::test_fn::extract_operational_base_addr;


pub trait ICreateCommandRingControl {
    fn new_command_ring_control(&self, operational_base_addr: u64) -> CreateRegisterResult<CommandRingControlRegister>;
}


impl ICreateCommandRingControl for CreateType {
    fn new_command_ring_control(&self, operational_base_addr: u64) -> CreateRegisterResult<CommandRingControlRegister> {
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


#[test_case]
pub fn should_new_command_ring_control() {
    let command_ring_control = uncheck_transmute(extract_operational_base_addr());
    
    assert!(command_ring_control.is_ok());
    serial_println!("{:?}", command_ring_control);
}
