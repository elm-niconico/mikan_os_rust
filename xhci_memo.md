# 目次



- [目次](#目次)
  - [Operators](#operators)
    - [UsbCmdRegister](#usbcmdregister)
    - [EnableWrapEvent(EWE)](#enablewrapeventewe)
  - [Microframe Index Register (MFINDEX)](#microframe-index-register-mfindex)


## Operators

### UsbCmdRegister

### EnableWrapEvent(EWE)

Bits 1  
デフォルト 0 

---- 

1に設定すると MFINDEXレジスタが03FFFhから0に遷移する度に、ラップイベントを生成します。

## Microframe Index Register (MFINDEX)

名 | 値
--|-----
Address | Runtime Base
Default Value  |0000 0000h
Attribute | RO
Size | 32bits

周期フレームを決めるためにソフトウェアによって使用されるレジスタ

このレジスタはUsbCmdのRun/Stop(R/s)が1の間インクリメントされます。
