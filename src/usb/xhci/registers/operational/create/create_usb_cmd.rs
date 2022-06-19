use crate::test_op_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operational::structs::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;


pub trait CreateUsbCommand {
    fn new_usb_command(&self, usb_cmd_addr: u64) -> CreateRegisterResult<UsbCmdRegister>;
}


impl CreateUsbCommand for CreateType {
    fn new_usb_command(&self, usb_cmd_addr: u64) -> CreateRegisterResult<UsbCmdRegister> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(usb_cmd_addr) }
        }
    }
}


fn uncheck_transmute(usb_cmd_addr: u64) -> CreateRegisterResult<UsbCmdRegister> {
    let usb_cmd = transmute_from_u64::<UsbCmdRegister>(usb_cmd_addr);
    //
    // if usb_cmd.host_controller_reset() {
    //     return Err(());
    // }
    // if usb_cmd.interrupt_enable() {
    //     return Err(());
    // }
    //
    // if usb_cmd.light_host_controller_reset() {
    //     return Err(());
    // }
    let info = RegisterInfo::new(usb_cmd_addr, usb_cmd);
    let volatile: Volatile<UsbCmdRegister> = Volatile::Core::<UsbCmdRegister>(info);
    return Ok(volatile);
}


test_op_register!(should_new_usb_cmd, uncheck_transmute);
