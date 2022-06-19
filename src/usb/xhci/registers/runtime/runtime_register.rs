use core::fmt::{Debug, Formatter};

#[repr(packed)]
pub struct RuntimeRegister {
    a: u64,
}
