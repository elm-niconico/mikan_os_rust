use crate::serial_println;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operators::structs::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointerRegister;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;
use crate::utils::test_fn::extract_operational_base_addr;


pub trait ICreateDeviceBaseAddressArrayPointer {
    fn new_device_context_base_address_array_pointer(&self, operational_base_addr: u64) -> CreateRegisterResult<DeviceContextBaseAddressArrayPointerRegister>;
}


impl ICreateDeviceBaseAddressArrayPointer for CreateType {
    fn new_device_context_base_address_array_pointer(&self, operational_base_addr: u64) -> CreateRegisterResult<DeviceContextBaseAddressArrayPointerRegister> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(operational_base_addr) }
            _ => { todo!() }
        }
    }
}


fn uncheck_transmute(operational_base_addr: u64) -> CreateRegisterResult<DeviceContextBaseAddressArrayPointerRegister> {
    let addr = operational_base_addr + 0x30;
    
    let device_context_base_address_array_pointer = transmute_from_u64::<>(addr);
    
    Ok(Volatile::Core(RegisterInfo::new(addr, device_context_base_address_array_pointer)))
}


#[test_case]
pub fn should_new_device_context_base_address() {
    let register = uncheck_transmute(extract_operational_base_addr());
    assert!(register.is_ok());
    serial_println!("{:?}", register.unwrap());
}
