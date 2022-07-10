use spin::mutex::{SpinMutex, SpinMutexGuard};

pub struct StaticSpinMutex<T>(SpinMutex<T>);


impl<T> StaticSpinMutex<T> {
    pub const fn new(value: T) -> Self {
        StaticSpinMutex(SpinMutex::new(value))
    }


    pub fn lock(&self) -> SpinMutexGuard<T> {
        self.0.lock()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
}