use core::arch::global_asm;

// 指定したアドレス(addr)に対してdataを書き込みます
// 今回の場合、コンフィグアドレスレジスタのポートアドレスに対し、
// コンフィギュレーション空間上の読み取り対象のアドレスを書きこんでいる(はず、多分)
global_asm!(
    ".global set_ds_all",
    "set_ds_all: ",
    "mov es, di",
    "mov fs, di",
    "mov gs, di",
    "ret"
);


global_asm!(
    ".global set_cs",
    "set_cs:",
        "push rbp",
        "mov rbp, rsp",
        "mov ss, si",
        "mov rax, .next",
        "push rdi",
        "push rax",
        "retf",
    ".next:",
        "mov rsp, rbp",
        "pop rbp",
        "ret"
);

extern "C" {
    // コンフィグアドレスレジスタにコンフィギュレーション空間の読み込み先のアドレスを書き込みます
    // addrはRDIレジスタに,dataはRSIレジスタに設定される
    pub fn set_ds_all(value: u16);

    pub fn set_cs(cs: u16, ss: u16);
}