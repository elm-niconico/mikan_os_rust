use x86_64::structures::paging::PhysFrame;

pub trait BitWritable {
    fn write_bit(&mut self, frame: PhysFrame, is_allocated: bool);
}