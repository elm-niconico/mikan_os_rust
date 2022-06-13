#[repr(C)]
pub struct OperationalRegisters {
    MemMapRegister<USBCMD_Bitmap> USBCMD;
    MemMapRegister<USBSTS_Bitmap> USBSTS;
    MemMapRegister<DefaultBitmap<uint32_t>> PAGESIZE;
    uint32_t reserved1[2];
    MemMapRegister<DefaultBitmap<uint32_t>> DNCTRL;
    MemMapRegister<CRCR_Bitmap> CRCR;
    uint32_t reserved2[4];
    MemMapRegister<DCBAAP_Bitmap> DCBAAP;
    MemMapRegister<CONFIG_Bitmap> CONFIG;
}



