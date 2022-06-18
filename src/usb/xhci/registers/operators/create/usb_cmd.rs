use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::operators::structs::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::{transmute_from_u64, transmute_register};
use crate::utils::test_fn::extract_operational_base_addr;


pub trait CreateUsbCommand {
    fn new_usb_command(&self, usb_cmd_addr: u64) -> Result<RegisterInfo<UsbCmdRegister>, &'static str>;
}


impl CreateUsbCommand for CreateType {
    fn new_usb_command(&self, usb_cmd_addr: u64) -> Result<RegisterInfo<UsbCmdRegister>, &'static str> {
        match self {
            CreateType::UncheckTransmute => { Ok(transmute_register::<UsbCmdRegister>(usb_cmd_addr)) }
            CreateType::TransmuteWithCheck => { transmute_with_check(usb_cmd_addr) }
        }
    }
}


fn transmute_with_check(usb_cmd_addr: u64) -> Result<RegisterInfo<UsbCmdRegister>, &'static str> {
    let usb_cmd = transmute_from_u64::<UsbCmdRegister>(usb_cmd_addr);
    
    if usb_cmd.host_controller_reset() {
        return Err("Value Of Host Controller Reset Expected False But True");
    }
    if usb_cmd.interrupt_enable() {
        return Err("Value Of Interrupt Enable Expected False But True");
    }
    
    if usb_cmd.light_host_controller_reset() {
        return Err("Value Of Light Host Controller Reset Expected False But True");
    }
    
    return Ok(RegisterInfo::new(usb_cmd_addr.clone(), usb_cmd));
}


#[test_case]
pub fn should_new_usb_command() {
    let usb_cmd = transmute_with_check(extract_operational_base_addr());
    assert!(usb_cmd.is_ok())
}
