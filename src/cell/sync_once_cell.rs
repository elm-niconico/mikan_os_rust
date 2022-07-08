use core::cell::OnceCell;
use core::fmt::Debug;

pub(crate) struct SyncOnceCell<T>(OnceCell<T>);

impl<T: Debug> SyncOnceCell<T> {
    pub const fn new() -> Self {
        SyncOnceCell(OnceCell::new())
    }

    pub fn set(&mut self, value: T) {
        unsafe { self.0.set(value) }.expect("Failed Set OnceCell")
    }
    pub fn get(&self) -> &T {
        self.0.get().unwrap()
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().unwrap()
    }
}

unsafe impl<T> Sync for SyncOnceCell<T> {}