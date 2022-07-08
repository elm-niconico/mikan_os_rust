use x86_64::structures::paging::PhysFrame;

pub trait BitReadable {
    fn read_bit(&self, frame: PhysFrame) -> bool;
}