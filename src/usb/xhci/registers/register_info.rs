use core::fmt::Debug;
use core::ops::{Deref, DerefMut};


#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct RegisterInfo<T>(*mut T);


impl<T: Debug> RegisterInfo<T> {
    pub fn new(register_addr: u64) -> Self {
        Self(register_addr as *mut T)
    }
    
    pub fn addr(&self) -> usize {
        self.0.addr()
    }
    
    pub fn ptr(&self) -> *mut T{
        self.0
    }
}



