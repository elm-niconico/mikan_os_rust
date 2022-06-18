#[macro_export]
macro_rules! test_cap_register {
    ($name: tt, $fn: ident) => {
        #[test_case]
        pub fn $name() {
            use crate::utils::test_fn::extract_virtual_mmio_base_addr;
            use crate::{serial_println};
            
            let register = $fn(extract_virtual_mmio_base_addr());
            assert!(register.is_ok());
            serial_println!("{:?}", register.unwrap());
        }
    };
}
