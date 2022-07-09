use core::fmt::Debug;

use conquer_once::spin::OnceCell;

pub(crate) struct StaticOnceCell<T>(OnceCell<T>);

impl<T: Debug> StaticOnceCell<T> {
    pub const fn uninit() -> Self {
        StaticOnceCell(OnceCell::uninit())
    }

    pub fn init_once(&mut self, init: impl FnOnce() -> T) {
        unsafe { self.0.init_once(init) };
    }
    pub fn get(&self) -> &T {
        unsafe { self.0.get_unchecked() }
    }
}

unsafe impl<T> Sync for StaticOnceCell<T> {}