use spin::mutex::SpinMutex;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::idt::InterruptStackFrame;

use crate::{FRAME_ALLOCATOR, serial_println};
use crate::error::kernel_error::KernelResult;
use crate::interrupt::apic::mouse::mouse_drawer::{MOUSE_CURSOR, MouseCursor};
use crate::interrupt::apic::timer::notify_end_of_interrupt;
use crate::interrupt::identity_mapping;
use crate::memory::frame::bit_map_manager::BYTES_PER_FRAME;
use crate::usb::pci::configuration::{configure_msi_fixed_destination, find_xhc_device, tmp_find_usb_mouse_base};
use crate::usb::xhci::controller::initialize::init_xhci;
use crate::usb::xhci::controller::lib_base_controller::LibBaseController;

mod mouse_drawer;

pub trait XhcMouseController {
    fn init(phys_offset: VirtAddr);
    fn run();
}


pub static XHC_MOUSE: conquer_once::noblock::OnceCell<SpinMutex<LibBaseController>> = conquer_once::noblock::OnceCell::uninit();

// 適当な値
const DEVICE_MAX_SLOTS: u8 = 8;

pub unsafe fn init(phys_offset: VirtAddr) {
    serial_println!("Start Find MMIO Base Address!");
    let xhc_mmio_base_addr = tmp_find_usb_mouse_base().unwrap();


    let bsp_local_apic_id = unsafe { *(0xfee00020 as *const u32) } >> 24;
    configure_msi_fixed_destination(&find_xhc_device().unwrap(), bsp_local_apic_id, true, 0b000, 0x40, 0);

    let mmio_base = PhysAddr::new(xhc_mmio_base_addr);
    // let mapper = &mut *PAGE_TABLE.get_unchecked().lock();
    //
    // identity_mapping(mapper, mmio_base.as_u64(), 16).unwrap();
    // alloc_memory_pool(mapper).unwrap();

    //let mut xhc_controller = unsafe { mikanos_usb::xhci::Controller::new(mmio_base.as_u64()) };
    let mut xhc_controller = LibBaseController::try_new(mmio_base, 1, phys_offset.as_u64()).unwrap();

    mikanos_usb::HidMouseDriver::set_default_observer(observer);

    init_xhci(&mut xhc_controller, 1).unwrap();
    xhc_controller.run();


    xhc_controller.configure_connected_ports();


    MOUSE_CURSOR.init_once(|| {
        let mut cursor = MouseCursor::new();
        cursor.init().expect("Failed To Init Mouse Cursor Draw");
        SpinMutex::new(cursor)
    });


    XHC_MOUSE.try_init_once(move || SpinMutex::new(xhc_controller)).expect("Failed Init Once Xhc Mouse");
}


// FIXME マウスのオブザーバーの処理を記述 Allowも消す
#[allow(unused)]
pub(crate) extern "C" fn observer(buttons: u8, displacement_x: i8, displacement_y: i8) {
    serial_println!("x : {} y: {}", displacement_x, displacement_y);


    MOUSE_CURSOR.get().lock().move_mouse(displacement_x as isize, displacement_y as isize);
}


fn alloc_memory_pool(mapper: &mut x86_64::structures::paging::OffsetPageTable) -> KernelResult<()> {
    let num_frames = 32;

    let frame_range = FRAME_ALLOCATOR.lock().allocate(num_frames)?;
    let base_addr = frame_range.start.start_address().as_u64();
    identity_mapping(mapper, base_addr, num_frames)?;
    unsafe { mikanos_usb::set_memory_pool(base_addr, num_frames * (BYTES_PER_FRAME as usize)) };
    Ok(())
}

pub extern "x86-interrupt" fn xhci_mouse_handler(stack_frame: InterruptStackFrame) {
    let mut mouse = unsafe { XHC_MOUSE.get_unchecked().lock() };
    // while mouse.has_event() {
    //     let _ = mouse
    //         .process_event()
    //         .map_err(|e| { serial_println!("Mouse Error {:?}", e.0); });
    // }

    serial_println!("has event? {}",mouse.has_event());

    notify_end_of_interrupt();
}