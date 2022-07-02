use crate::serial_println;
use crate::usb_my::xhci::registers::create_type::RegisterCreate;
use crate::usb_my::xhci::registers::runtime::structs::runtime_registers::InterruptersArray;

// 1024
pub const INTERRUPTER_SET_COUNTS: usize = 1024;

pub trait ICreateInterrupterRegisterSet {
    fn new_interrupter_register_set(&self, runtime_base: u64) -> InterruptersArray;
}

impl ICreateInterrupterRegisterSet for RegisterCreate {
    fn new_interrupter_register_set(&self, runtime_base: u64) -> InterruptersArray {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(runtime_base),
        }
    }
}

fn uncheck_transmute(runtime_base: u64) -> InterruptersArray {
    InterruptersArray::new(runtime_base)
}

// fn uncheck_transmute(runtime_base: u64) -> CreateInterrupterRegisterSetResult {
//     let base = runtime_base + 0x20;
//     let ptr = base as *const InterrupterRegisterSet;
//
//     let interrupters_unknown_size = unsafe { slice::from_raw_parts(ptr, INTERRUPTER_SET_COUNTS) };
//     let interrupters = pop(interrupters_unknown_size);
//
//
//     // TODO ここめっちゃ怪しい
//     interrupters.map(|interrupt| {
//         let addr = (&interrupt as *const InterrupterRegisterSet).addr() as u64;
//
//         let a = Volatile::Core(RegisterInfo::new(addr, interrupt));
//
//         a
//     })
// }

// #[test_case]
// pub fn should_new_interrupter_sets() {
//     use crate::utils::test_fn::extract_runtime_base;
//     let interrupters = uncheck_transmute(extract_runtime_base());
//     //assert_eq!(1024, interrupters.);
// }