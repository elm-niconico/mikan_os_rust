use crate::test_op_register;
use crate::usb_my::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb_my::xhci::registers::operational::structs::command_ring_control::CommandRingControlRegister;
use crate::utils::raw_ptr::transmute_register;

pub trait ICreateCrcr {
    /** New Command Ring Control Register */
    fn new_crcr(
        &self,
        operational_base_addr: u64,
    ) -> CreateRegisterResult<CommandRingControlRegister>;
}

impl ICreateCrcr for RegisterCreate {
    fn new_crcr(
        &self,
        operational_base_addr: u64,
    ) -> CreateRegisterResult<CommandRingControlRegister> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(operational_base_addr),
        }
    }
}

fn uncheck_transmute(
    operational_base_addr: u64,
) -> CreateRegisterResult<CommandRingControlRegister> {
    let addr = operational_base_addr + 0x18;
    Ok(transmute_register(addr))
}

test_op_register!(should_new_crcr, uncheck_transmute);