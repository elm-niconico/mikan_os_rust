pub use crate::error::error::*;

mod error;


pub type KernelResult<T> = Result<T, KernelError>;