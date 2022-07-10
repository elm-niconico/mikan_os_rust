use core::fmt::Arguments;

use spin::mutex::Mutex;
use uart_16550::SerialPort;
use x86_64::instructions::interrupts::without_interrupts;

pub static SERIAL: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });


pub fn init() {
    let mut serial_port = SERIAL.lock();
    serial_port.init();
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;

    without_interrupts(|| {
        SERIAL
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}