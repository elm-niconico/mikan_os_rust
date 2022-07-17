use spin::mutex::SpinMutex;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::idt::InterruptStackFrame;

use mikanos_usb::xhci::Controller;

use crate::{FRAME_ALLOCATOR, PAGE_TABLE, serial_println, tmp_find_usb_mouse_base};
use crate::error::KernelResult;
use crate::interrupt::apic::mouse::mouse_drawer::{MOUSE_CURSOR, MouseCursor};
use crate::interrupt::apic::timer::notify_end_of_interrupt;
use crate::interrupt::identity_mapping;
use crate::memory::frame::bit_map_manager::BYTES_PER_FRAME;
use crate::usb::pci::configuration::{configure_msi_fixed_destination, find_xhc_device};

mod mouse_drawer;

pub trait XhcMouseController {
    fn init(phys_offset: VirtAddr);
    fn run();
}


pub static XHC_MOUSE: conquer_once::noblock::OnceCell<SpinMutex<&'static mut Controller>> = conquer_once::noblock::OnceCell::uninit();

// 適当な値
const DEVICE_MAX_SLOTS: u8 = 8;

pub unsafe fn init(phys_offset: VirtAddr) {
    serial_println!("Start Find MMIO Base Address!");
    let xhc_mmio_base_addr = tmp_find_usb_mouse_base().unwrap();


    let bsp_local_apic_id = unsafe { *(0xfee00020 as *const u32) } >> 24;
    configure_msi_fixed_destination(&find_xhc_device().unwrap(), bsp_local_apic_id, true, 0b000, 0x40, 0);

    let dev = find_xhc_device().unwrap();


    let mmio_base = PhysAddr::new(xhc_mmio_base_addr);

    let mapper = &mut *PAGE_TABLE.get_unchecked().lock();
    identity_mapping(mapper, mmio_base.as_u64(), 16).unwrap();
    alloc_memory_pool(mapper).unwrap();
    let mut con = unsafe { mikanos_usb::xhci::Controller::new(mmio_base.as_u64()) };

    mikanos_usb::HidMouseDriver::set_default_observer(observer);

    con.init();
    let run = con.run();
    assert!(run.is_ok());
    con.configure_connected_ports();

    serial_println!("Mapped Mmio Base");


    MOUSE_CURSOR.init_once(||{
       let mut cursor = MouseCursor::new();
        cursor.init();
        return SpinMutex::new(cursor);
    });

    //let bsp_local_apic_id: u8 = unsafe { ptr::read_volatile(0xfee00020 as *mut u32).get_bits(24..32) } as u8;
    // let bsp_local_apic_id = unsafe { *(0xfee00020 as *const u32) } >> 24;
    //
    // let device = find_xhc_device().unwrap();
    //
    // configure_msi_fixed_destination(&device, bsp_local_apic_id as u32, true, 0, InterruptIndex::Xhci.as_u8(), 0);
    // log!("Configure Msi Fixed");
    //
    //
    // FIXME このメソッドが呼ばれる瞬間に落ちている
    // let mut con = LibBaseController::try_new(mmio_base, DEVICE_MAX_SLOTS).expect("Failed New Xhci");
    // serial_println!("New Xhci");
    //
    // init_xhci(&mut con, DEVICE_MAX_SLOTS, phys_offset).expect("Failed Initialize Xhc Controller");
    // serial_println!("Init Xhci");

    XHC_MOUSE.try_init_once(move || SpinMutex::new(con)).expect("Failed Init Once Xhc Mouse");
    serial_println!("Once Cell Init Xhci");
}

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
    while mouse.has_event() {
        let _ = mouse
            .process_event()
            .map_err(|e| { serial_println!("Mouse Error {:?}", e.0); });
    }

    notify_end_of_interrupt();
}