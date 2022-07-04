mod error;

pub use error::KernelError;

pub type KernelResult<T> = Result<T, KernelError>;