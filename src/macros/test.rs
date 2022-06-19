#[macro_export]
macro_rules! test_cap_register {
    ($name: tt, $fn: ident) => {
        use crate::utils::test_fn::extract_virtual_mmio_base_addr;
        use crate::test_register;
        
        test_register!($name, $fn, extract_virtual_mmio_base_addr());
    };
}

#[macro_export]
macro_rules! test_op_register {
    ($name: tt, $fn: ident) => {
        use crate::utils::test_fn::extract_operational_base_addr;
        use crate::test_register;
        
        test_register!($name, $fn, extract_operational_base_addr());
    };
}

#[macro_export]
macro_rules! test_runtime_register {
    ($name: tt, $fn: ident) => {
        use crate::utils::test_fn::extract_runtime_base;
        use crate::test_register;
        
        test_register!($name, $fn, extract_runtime_base());
    };
}

#[macro_export]
macro_rules! test_register {
    ($name: tt, $fn: ident, $addr: expr) => {
        #[test_case]
        pub fn $name() {
            use crate::utils::test_fn::extract_virtual_mmio_base_addr;
            
            let register = $fn($addr);
            assert!(register.is_ok());
            crate::serial_println!("{:?}", register.unwrap());
        }
    };
}
