use core::fmt::Arguments;

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use x86_64::instructions::interrupts::without_interrupts;

lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = new_serial_port();
}

fn new_serial_port() -> Mutex<SerialPort> {
    //最初のシリアルインターフェースの標準のポート番号
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    Mutex::new(serial_port)
}


#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;

    without_interrupts(|| {
        SERIAL
            .lock()
            .write_fmt(args)
            .expect("Failed Print To Serial");
    });
}