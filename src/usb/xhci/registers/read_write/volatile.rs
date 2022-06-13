use crate::usb::xhci::registers::r#struct::register_info::RegisterInfo;


pub trait IVolatile<T> {
    fn read_volatile(&self) -> T;
    fn write_volatile(&mut self, src: T);
    fn update_volatile(&mut self, update_fn: fn(r: &mut T));
}


pub enum Volatile<T> {
    Core(RegisterInfo<T>)
}


impl<T> IVolatile<T> for Volatile<T> {
    fn read_volatile(&self) -> T {
        match self {
            Volatile::Core(r) => {
                unsafe { core::ptr::read_volatile(r.get_register_raw_ptr()) }
            }
        }
    }
    
    fn write_volatile(&mut self, src: T) {
        match self {
            Volatile::Core(r) => {
                unsafe { core::ptr::write_volatile(r.get_register_raw_ptr(), src); };
            }
        }
    }
    
    fn update_volatile(&mut self, update_fn: fn(&mut T)) {
        let mut r = self.read_volatile();
        update_fn(&mut r);
        self.write_volatile(r);
    }
}
