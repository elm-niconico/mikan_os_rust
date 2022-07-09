use core::arch::global_asm;

global_asm!(
    ".global sti",
    "sti:",
    "sti",
    "ret"
);
global_asm!(
    ".global cli",
    "cli:",
    "cli",
    "ret"
);
extern "C" {
    /// 割り込みを停止させます
    pub fn cli();


    /// 割り込みを許可します
    pub fn sti();
}