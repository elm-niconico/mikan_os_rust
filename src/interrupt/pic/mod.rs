use pic8259::ChainedPics;
use spin::mutex::SpinMutex;

pub mod timer;

pub static PICS: SpinMutex<ChainedPics> = unsafe { SpinMutex::new(ChainedPics::new(0x20, 0x28)) };