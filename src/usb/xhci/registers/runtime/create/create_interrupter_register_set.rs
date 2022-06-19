use core::slice;

use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::usb::xhci::registers::runtime::structs::interrupter::interrupter_registers::InterrupterRegisterSet;
use crate::utils::test_fn::extract_runtime_base;


pub type CreateInterrupterRegisterSetResult = [Volatile<InterrupterRegisterSet>; 1024];


pub trait ICreateInterrupterRegisterSet {
    fn new_interrupter_register_set(&self, runtime_base: u64) -> CreateInterrupterRegisterSetResult;
}


impl ICreateInterrupterRegisterSet for CreateType {
    fn new_interrupter_register_set(&self, runtime_base: u64) -> CreateInterrupterRegisterSetResult {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(runtime_base) }
        }
    }
}


fn uncheck_transmute(runtime_base: u64) -> CreateInterrupterRegisterSetResult {
    let base = runtime_base + 0x20;
    let addr = base as *const InterrupterRegisterSet;
    let interrupters_unknown_size = unsafe { slice::from_raw_parts(addr, 1024) };
    let interrupters = pop(interrupters_unknown_size);
    interrupters.map(|interrupt| {
        let addr = (&interrupt as *const InterrupterRegisterSet).addr() as u64;
        Volatile::Core(RegisterInfo::new(addr, interrupt))
    })
}


fn pop(barry: &[InterrupterRegisterSet]) -> &[InterrupterRegisterSet; 1024] {
    barry.try_into().expect("slice with incorrect length")
}


#[test_case]
pub fn should_new_interrupter_sets() {
    let interrupters = uncheck_transmute(extract_runtime_base());
    assert_eq!(1024, interrupters.len());
}
