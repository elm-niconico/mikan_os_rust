use core::ptr;

use bit_field::BitField;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::VirtAddr;

use crate::{log, print, tmp_find_usb_mouse_base};
use crate::usb::xhci::controller::lib_base_controller::LibBaseController;
use crate::interrupt::apic::timer::notify_end_of_interrupt;
use crate::interrupt::idt::InterruptIndex;
use crate::spin::sync_mutex::StaticSpinMutex;
use crate::spin::sync_once_cell::StaticOnceCell;
use crate::usb::pci::configuration::{configure_msi_fixed_destination, find_xhc_device};
use crate::usb::xhci::controller::zero_base_controller::ZeroBaseController;


pub trait XhcMouseController {
    fn init(phys_offset: VirtAddr);
    fn run();
}

pub static XHC_MOUSE: StaticOnceCell<StaticSpinMutex<LibBaseController>> = StaticOnceCell::uninit();

pub fn init(phys_offset: VirtAddr, rsdp: VirtAddr) {
    log!("Start Find MMIO Base Address!");
    let mmio_base = tmp_find_usb_mouse_base().unwrap();
    log!("MMIO Base Address {}", mmio_base);
    //let bsp_local_apic_id: u8 = unsafe { ptr::read_volatile(0xfee00020 as *mut u32).get_bits(24..32) } as u8;
    let bsp_local_apic_id = unsafe { *(0xfee00020 as *const u32) } >> 24;
    let device = find_xhc_device().unwrap();
    configure_msi_fixed_destination(&device, bsp_local_apic_id as u32, true, 0, InterruptIndex::Xhci.as_u8(), 0);

    XHC_MOUSE.init_once(|| StaticSpinMutex::new(LibBaseController::new(phys_offset)))
}


pub extern "x86-interrupt" fn xhci_mouse_handler(stack_frame: InterruptStackFrame) {
    print!("mouse");

    notify_end_of_interrupt();
}