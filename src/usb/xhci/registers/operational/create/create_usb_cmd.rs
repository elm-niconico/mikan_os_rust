use crate::test_op_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::operational::structs::usb_cmd::UsbCmdRegister;
use crate::utils::raw_ptr::transmute_register;

pub trait CreateUsbCommand {
    fn new_usb_command(&self, usb_cmd_addr: u64) -> CreateRegisterResult<UsbCmdRegister>;
}

impl CreateUsbCommand for RegisterCreate {
    fn new_usb_command(&self, usb_cmd_addr: u64) -> CreateRegisterResult<UsbCmdRegister> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(usb_cmd_addr),
        }
    }
}

fn uncheck_transmute(usb_cmd_addr: u64) -> CreateRegisterResult<UsbCmdRegister> {
    Ok(transmute_register(usb_cmd_addr))
}

test_op_register!(should_new_usb_cmd, uncheck_transmute);