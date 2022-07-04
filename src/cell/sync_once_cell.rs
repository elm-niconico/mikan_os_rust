use core::lazy::OnceCell;

pub(crate) struct SyncOnceCell<T>(OnceCell<T>);

impl<T> SyncOnceCell<T> {
    pub const fn new() -> Self {
        SyncOnceCell(OnceCell::new())
    }

    pub fn set(&mut self, value: T) -> Result<(), ()> {
        unsafe { self.0.set(value) }.map_err(|r| ())
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.0.get_mut()
    }
}

unsafe impl<T> Sync for SyncOnceCell<T> {}