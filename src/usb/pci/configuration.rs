use crate::assembly::configuration::{io_in_32, io_out_32};
use crate::usb::pci::capability_header::{CapabilityHeader, MsiCapability};

pub struct ConfigRegisterFinder {
    pub devices: [Option<Device>; 32],
    pub device_count: usize,
}

impl ConfigRegisterFinder {
    pub fn new() -> Self {
        Self {
            devices: [Option::None; 32],
            device_count: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Device {
    pub bus: u32,
    pub device: u32,
    pub func: u32,
    pub header_type: u8,
    pub class_code: ClassCode,
}

#[derive(Copy, Clone, Debug)]
pub struct ClassCode {
    base: u8,
    sub: u8,
    interface: u8,
}

impl ClassCode {
    pub fn is_match(&self, base: u8, sub: u8, interface: u8) -> bool {
        (base == self.base) && (sub == self.sub) && (interface == self.interface)
    }
}

impl ConfigRegisterFinder {
    pub fn scan_all_device(&mut self) -> Result<(), ()> {
        let host_header = read_header_type(0, 0, 0);
        if is_single_function_device(host_header) {
            self.scan_bus(0);
            Ok(())
        } else {
            Err(())
        }
    }

    fn scan_bus(&mut self, bus: u32) {
        for device in 0..32 {
            if read_vendor_id(bus, device, 0) == 0xFFFF {
                continue;
            }
            self.scan_device(bus, device);
        }
    }

    fn scan_device(&mut self, bus: u32, device: u32) {
        let div = self.scan_function(bus, device, 0);
        self.add_device(div);
        if is_single_function_device(read_header_type(bus, device, 0)) {
            return;
        }

        for func in 1..8 {
            self.scan_function(bus, device, func);
        }
    }

    pub fn scan_function(&mut self, bus: u32, device: u32, func: u32) -> Device {
        let class_code = read_class_code(bus, device, func);
        let header_type = read_header_type(bus, device, func);
        let dev = Device {
            bus,
            device,
            func,
            header_type,
            class_code,
        };
        if class_code.base == 0x06 && class_code.sub == 0x04 {
            let bus_numbers = read_bus_numbers(bus, device, func);
            let secondary_bus = (bus_numbers >> 8) & 0xFF;
            self.scan_bus(secondary_bus);
        }
        return dev;
    }

    fn add_device(&mut self, device: Device) {
        if self.devices.len() == self.device_count {
            return;
        }

        self.devices[self.device_count] = Some(device);
        self.device_count += 1;
    }
}

static CONFIG_ADDRESS: u16 = 0x0CF8;

static CONFIG_DATA: u16 = 0x0CFC;

#[allow(unused)]
pub fn write_address(data: u32) {
    unsafe { io_out_32(CONFIG_ADDRESS, data) };
}

#[allow(unused)]
pub fn write_data(value: u32) {
    unsafe { io_out_32(CONFIG_DATA, value) };
}

#[allow(unused)]
pub fn read_vendor_id(bus: u32, device: u32, func: u32) -> u32 {
    write_address(make_address(bus, device, func, 0x00));

    read_data() & 0xFFFF
}

#[allow(unused)]
pub fn read_vendor_id_from_dev(dev: &Device) -> u32 {
    read_vendor_id(dev.bus, dev.device, dev.func)
}

#[allow(unused)]
pub fn read_class_code(bus: u32, device: u32, func: u32) -> ClassCode {
    write_address(make_address(bus, device, func, 0x08));
    let data = read_data();
    ClassCode {
        base: (data >> 24) as u8,
        sub: ((data >> 16) & 0xFF) as u8,
        interface: ((data >> 8) & 0xFF) as u8,
    }
}

pub fn read_header_type(bus: u32, device: u32, func: u32) -> u8 {
    write_address(make_address(bus, device, func, 0x0C));

    ((read_data() >> 16) & 0xFF) as u8
}

pub fn read_bus_numbers(bus: u32, device: u32, func: u32) -> u32 {
    write_address(make_address(bus, device, func, 0x18));
    read_data()
}

pub fn is_single_function_device(header_type: u8) -> bool {
    (header_type & 0x80) == 0
}

pub fn read_data() -> u32 {
    unsafe { io_in_32(CONFIG_DATA) }
}

pub fn read_cong(device: &Device, addr: u32) -> u32 {
    write_address(make_address(device.bus, device.device, device.func, addr));
    read_data()
}

pub fn read_bar(device: &Device, index: u32) -> Result<u64, ()> {
    let addr = 0x10 + 4 * index;
    let bar = read_cong(device, addr);

    // 32bit
    if (bar & 0x4) == 0 {
        return Ok(bar as u64);
    }

    let upper_bar: u64 = read_cong(device, addr + 4) as u64;
    let result: u64 = (upper_bar) << 32 | bar as u64;
    Ok(result)
}

fn make_address(bus: u32, device: u32, func: u32, reg_addr: u32) -> u32 {
    let shl = |x: u32, bits: usize| -> u32 { (x << bits) as u32 };

    let addr: u32 =
        shl(1, 31) | shl(bus, 16) | shl(device, 11) | shl(func, 8) | (reg_addr & 0xFC) as u32;
    addr as u32
}

pub fn find_xhc_device() -> Option<Device> {
    let mut config_register = ConfigRegisterFinder::new();
    config_register.scan_all_device().expect("failed scan");

    for i in 0..config_register.device_count {
        let dev = config_register.devices[i];
        if let Some(device) = dev {
            let is_xhci = device.class_code.is_match(0x0C, 0x03, 0x30);
            if is_xhci {
                return Some(device);
            }
        }
    }
    None
}

fn to_base_bar(bar: u64) -> u64 {
    let mask: u64 = 0xf;

    bar & !mask
}

#[allow(unused)]
const CAPABILITY_MSI: u8 = 0x05;

#[allow(unused)]
const CAPABILITY_MSI_X: u8 = 0x11;

pub fn tmp_find_usb_mouse_base() -> Result<u64, ()> {
    let xhc_dev = find_xhc_device();

    return if let Some(xhc_dev) = xhc_dev {
        let base_bus = read_bar(&xhc_dev, 0).unwrap();

        let xhc_mmio_base = to_base_bar(base_bus);
        Ok(xhc_mmio_base)
    } else {
        Err(())
    };
}

pub fn read_conf_reg(dev: &Device, reg_addr: u8) -> u32 {
    write_address(make_address(dev.bus, dev.device, dev.func, reg_addr as u32));
    read_data()
}

#[allow(unused)]
fn read_capability_header(dev: &Device, addr: u32) -> CapabilityHeader {
    CapabilityHeader::try_from(read_conf_reg(dev, addr as u8)).unwrap()
}

#[allow(unused)]
pub fn configure_msi_fixed_destination(
    dev: &Device,
    apic_id: u32,
    trigger_mode: bool,
    delivery_mode: u32,
    vector: u8,
    num_vector_exponent: usize,
) {
    let msg_addr: u32 = (0xfee00000 as u32 | (apic_id << 12) as u32) as u32;
    let mut msg_data: u32 = ((delivery_mode << 8) as u32 | vector as u32) as u32;
    if trigger_mode {
        msg_data |= 0xc000;
    }

    configure_msi(dev, msg_addr, msg_data, num_vector_exponent);
}

#[allow(unused)]
fn configure_msi(dev: &Device, msg_addr: u32, msg_data: u32, num_vector_exponent: usize) {
    let mut cap_addr = read_conf_reg(dev, 0x34) & 0xff;
    let mut msi_cap_addr: u32 = 0;
    let mut msix_cap_addr = 0;
    while cap_addr != 0 {
        let header = read_capability_header(dev, cap_addr);
        if header.cap_id() == CAPABILITY_MSI {
            msi_cap_addr = cap_addr;
        } else if header.cap_id() == CAPABILITY_MSI_X {
            msix_cap_addr = cap_addr;
        }
        cap_addr = header.next_ptr() as u32;
    }

    if msi_cap_addr != 0 {
        unsafe {
            configure_msi_register(dev, msi_cap_addr, msg_addr, msg_data, num_vector_exponent);
        };
    } else if msix_cap_addr != 0 {
        //ConfigureMSIXRegister(dev, msix_cap_addr, msg_addr, msg_data, num_vector_exponent);
    }
}

/** @brief 指定された MSI レジスタを設定する */
#[allow(unused)]
unsafe fn configure_msi_register(
    dev: &Device,
    cap_addr: u32,
    msg_addr: u32,
    msg_data: u32,
    num_vector_exponent: usize,
) {
    let mut msi_cap = read_msi_capability(dev, cap_addr as u8);

    if msi_cap.header.bits.multi_msg_capable() <= num_vector_exponent as u8 {
        msi_cap
            .header
            .bits
            .set_multi_msg_enable(msi_cap.header.bits.multi_msg_capable());
    } else {
        msi_cap
            .header
            .bits
            .set_multi_msg_enable(num_vector_exponent as u8);
    }

    msi_cap.header.bits.set_msi_enable(true);
    msi_cap.msg_addr = msg_addr;
    msi_cap.msg_data = msg_data;

    write_msi_capability(dev, cap_addr, &msi_cap);
}

#[allow(unused)]
unsafe fn read_msi_capability(dev: &Device, cap_addr: u8) -> MsiCapability {
    let mut msi_cap = MsiCapability::new(dev, cap_addr);
    msi_cap.msg_addr = read_conf_reg(dev, cap_addr + 4);

    let mut msg_data_addr = cap_addr + 8;
    if msi_cap.header.bits.addr_64_capable() {
        msi_cap.msg_upper_addr = read_conf_reg(dev, cap_addr + 8);
        msg_data_addr = cap_addr + 12;
    }

    msi_cap.msg_data = read_conf_reg(dev, msg_data_addr);

    if msi_cap.header.bits.per_vector_mask_capable() {
        msi_cap.mask_bits = read_conf_reg(dev, msg_data_addr + 4);
        msi_cap.pending_bits = read_conf_reg(dev, msg_data_addr + 8);
    }

    return msi_cap;
}

#[allow(unused)]
unsafe fn write_msi_capability(dev: &Device, cap_addr: u32, msi_cap: &MsiCapability) {
    write_conf_reg(dev, cap_addr, msi_cap.header.data);
    write_conf_reg(dev, cap_addr + 4, msi_cap.msg_addr);

    let mut msg_data_addr = cap_addr + 8;
    if msi_cap.header.bits.addr_64_capable() {
        write_conf_reg(dev, cap_addr + 8, msi_cap.msg_upper_addr);
        msg_data_addr = cap_addr + 12;
    }

    write_conf_reg(dev, msg_data_addr, msi_cap.msg_data);

    if msi_cap.header.bits.per_vector_mask_capable() {
        write_conf_reg(dev, msg_data_addr + 4, msi_cap.mask_bits);
        write_conf_reg(dev, msg_data_addr + 8, msi_cap.pending_bits);
    }
}

#[allow(unused)]
fn write_conf_reg(dev: &Device, reg_addr: u32, value: u32) {
    write_address(make_address(dev.bus, dev.device, dev.func, reg_addr));
    write_data(value);
}