use conquer_once::spin::OnceCell;

pub struct StaticOnceCell<T>(OnceCell<T>);

impl<T> StaticOnceCell<T> {
    pub const fn uninit() -> Self {
        StaticOnceCell(OnceCell::uninit())
    }

    pub fn init_once(&self, init: impl FnOnce() -> T) {
        unsafe { self.0.init_once(init) };
    }
    pub fn get(&self) -> &T {
        unsafe { self.0.get_unchecked() }
    }
}

unsafe impl<T> Sync for StaticOnceCell<T> {

}