use spin::mutex::SpinMutex;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::idt::InterruptStackFrame;

use crate::{log, print, serial_println, tmp_find_usb_mouse_base};
use crate::interrupt::apic::timer::notify_end_of_interrupt;
use crate::interrupt::idt::InterruptIndex;
use crate::usb::pci::configuration::{configure_msi_fixed_destination, find_xhc_device};
use crate::usb::xhci::controller::initialize::init_xhci;
use crate::usb::xhci::controller::lib_base_controller::LibBaseController;

pub trait XhcMouseController {
    fn init(phys_offset: VirtAddr);
    fn run();
}


pub static XHC_MOUSE: conquer_once::noblock::OnceCell<SpinMutex<LibBaseController>> = conquer_once::noblock::OnceCell::uninit();

// 適当な値
const DEVICE_MAX_SLOTS: u8 = 8;

pub fn init(phys_offset: VirtAddr) {
    serial_println!("Start Find MMIO Base Address!");
    let xhc_mmio_base_addr = tmp_find_usb_mouse_base().unwrap();
    let mmio_base = PhysAddr::new(xhc_mmio_base_addr);

    serial_println!("Mapped Mmio Base");

    //let bsp_local_apic_id: u8 = unsafe { ptr::read_volatile(0xfee00020 as *mut u32).get_bits(24..32) } as u8;
    let bsp_local_apic_id = unsafe { *(0xfee00020 as *const u32) } >> 24;

    let device = find_xhc_device().unwrap();

    configure_msi_fixed_destination(&device, bsp_local_apic_id as u32, true, 0, InterruptIndex::Xhci.as_u8(), 0);
    log!("Configure Msi Fixed");


    // FIXME このメソッドが呼ばれる瞬間に落ちている
    let mut xhc_controller = LibBaseController::try_new(mmio_base, DEVICE_MAX_SLOTS).expect("Failed New Xhci");
    serial_println!("New Xhci");

    init_xhci(&mut xhc_controller, DEVICE_MAX_SLOTS, phys_offset).expect("Failed Initialize Xhc Controller");
    serial_println!("Init Xhci");

    XHC_MOUSE.try_init_once(move || SpinMutex::new(xhc_controller)).expect("Failed Init Once Xhc Mouse");
    serial_println!("Once Cell Init Xhci");
}


pub extern "x86-interrupt" fn xhci_mouse_handler(stack_frame: InterruptStackFrame) {
    print!("mouse");

    notify_end_of_interrupt();
}