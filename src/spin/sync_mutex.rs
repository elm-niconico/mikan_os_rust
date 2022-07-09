use core::fmt::Debug;

use spin::mutex::{SpinMutex, SpinMutexGuard};

#[derive(Debug)]
pub struct StaticSpinMutex<T>(SpinMutex<T>);

impl<T: Debug> StaticSpinMutex<T> {
    pub const fn new(value: T) -> Self {
        StaticSpinMutex(SpinMutex::new(value))
    }


    pub fn lock(&self) -> SpinMutexGuard<T> {
        self.0.lock()
    }
}