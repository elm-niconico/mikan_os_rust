use core::fmt::Debug;

use crate::impl_debug_with_generic;


#[derive(Clone, Copy)]
pub struct RegisterInfo<T: Debug> {
    register: T,
    register_start_addr: u64,
}


impl<T: core::fmt::Debug> RegisterInfo<T> {
    pub fn new(register_start_addr: u64, register: T) -> Self {
        Self {
            register,
            register_start_addr,
        }
    }
    
    
    pub fn get_register_raw_ptr(&self) -> *mut T {
        self.register_start_addr as *mut T
    }
}


impl_debug_with_generic! {
    RegisterInfo{
        register,
        register_start_addr
    }
}
