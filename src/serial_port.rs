use core::fmt::Arguments;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = new_serial_port();
}

fn new_serial_port() -> Mutex<SerialPort> {
    //最初のシリアルインターフェースの標準のポート番号
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    Mutex::new(serial_port)
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial_port::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => {$crate::serial_print!("\n")};
    ($fmt:expr) => {$crate::serial_print!(concat!($fmt, "\n"))};
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;

    SERIAL
        .lock()
        .write_fmt(args)
        .expect("Failed Print To Serial");
}
