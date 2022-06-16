use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::operators::structs::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::{transmute_from_u64, transmute_register};


pub trait CreateUsbCommand {
    fn new_usb_command(&self, mmio_base: u64) -> Result<RegisterInfo<UsbCmdRegister>, &'static str>;
}


impl CreateUsbCommand for CreateType {
    fn new_usb_command(&self, mmio_base: u64) -> Result<RegisterInfo<UsbCmdRegister>, &'static str> {
        match self {
            CreateType::UncheckTransmute => { Ok(transmute_register::<UsbCmdRegister>(mmio_base)) }
            CreateType::TransmuteWithCheck => { transmute_with_check(mmio_base) }
        }
    }
}


fn transmute_with_check(mmio_base: u64) -> Result<RegisterInfo<UsbCmdRegister>, &'static str> {
    let usb_cmd = transmute_from_u64::<UsbCmdRegister>(mmio_base);
    
    if usb_cmd.host_controller_reset() {
        return Err("Value Of Host Controller Reset Expected False But It True");
    }
    if usb_cmd.interrupt_enable() {
        return Err("Value Of Interrupt Enable Expected False But It True");
    }
    
    if usb_cmd.light_host_controller_reset() {
        return Err("Value Of Light Host Controller Reset Expected False But It True");
    }
    
    return Ok(RegisterInfo::new(mmio_base.clone(), usb_cmd));
}
