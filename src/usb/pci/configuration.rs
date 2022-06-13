use crate::asm_func::{io_in_32, io_out_32};

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
    header_type: u8,
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

pub fn write_address(data: u32) {
    unsafe { io_out_32(CONFIG_ADDRESS, data) };
}

pub fn read_vendor_id(bus: u32, device: u32, func: u32) -> u32 {
    write_address(make_address(bus, device, func, 0x00));

    read_data() & 0xFFFF
}

pub fn read_vendor_id_from_dev(dev: &Device) -> u32 {
    read_vendor_id(dev.bus, dev.device, dev.func)
}

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

fn find_xhc_device() -> Option<Device> {
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

pub fn tmp_find_usb_mouse_base() -> Result<u64, ()> {
    let xhc_dev = find_xhc_device();

    if let Some(xhc_dev) = xhc_dev {
        let base_bus = read_bar(&xhc_dev, 0).unwrap();

        let xhc_mmio_base = to_base_bar(base_bus);
        return Ok(xhc_mmio_base);
    } else {
        return Err(());
    }
}
