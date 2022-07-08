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



extern "C" {
    // コンフィグアドレスレジスタにコンフィギュレーション空間の読み込み先のアドレスを書き込みます
    // addrはRDIレジスタに,dataはRSIレジスタに設定される
    pub fn set_ds_all(value: u16);
}