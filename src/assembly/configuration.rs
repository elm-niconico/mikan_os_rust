use core::arch::global_asm;

// 指定したアドレス(addr)に対してdataを書き込みます
// 今回の場合、コンフィグアドレスレジスタのポートアドレスに対し、
// コンフィギュレーション空間上の読み取り対象のアドレスを書きこんでいる(はず、多分)
// pub fn io_out_32(addr: u8, data: u32)
// dx = addr
// eax = data
global_asm!(
    ".global io_out_32",
    "io_out_32:",
    "mov dx, di",
    "mov eax, esi",
    "out dx, eax",
    "ret"
);

// コンフィギュレーション空間から値を読み込む
// 引数にはコンフィグデータレジスタのポートアドレスを指定
// コンフィグデータレジスタはコンフィグアドレスレジスタに書き込まれているアドレスをもとに
// コンフィギュレーション空間の値を読み込む
global_asm!(
    ".global io_in_32",
    "io_in_32:",
    "mov dx, di",
    "in eax, dx",
    "ret"
);

extern "C" {
    // コンフィグアドレスレジスタにコンフィギュレーション空間の読み込み先のアドレスを書き込みます
    // addrはRDIレジスタに,dataはRSIレジスタに設定される
    pub fn io_out_32(addr: u16, data: u32) -> u32;


    // コンフィギュレーション空間の値を読み込みます
    // 読み込む前にio_out_32で読み込みたいデータの先頭アドレスを指定する必要があります
    pub fn io_in_32(addr: u16) -> u32;
}