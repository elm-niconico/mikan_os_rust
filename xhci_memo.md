# 目次

- [目次](#目次)
  - [USBリンク](#usbリンク)
  - [Wake Up Events](#wake-up-events)
  - [Operators](#operators)
    - [UsbCmdRegister](#usbcmdregister)
    - [UsbStatusRegister(USBSTS)](#usbstatusregisterusbsts)
      - [HcHalted(0)](#hchalted0)
      - [Rsvdz(1)](#rsvdz1)
      - [HostSystemError(2)](#hostsystemerror2)
    - [EnableWrapEvent(EWE)](#enablewrapeventewe)
  - [Microframe Index Register (MFINDEX)](#microframe-index-register-mfindex)
  - [Device Context](#device-context)
  - [Slot Context](#slot-context)

## USBリンク

状態名 | 状態
------|-----
U1 | 高速終了
U2 | 低速の出口でスタンバイ
U3 | デバイスは中断状態になる必要がある

U1 -> U2 -> U3に段階的に変更する必要があるらしい？

[マイクロソフトのURL](https://docs.microsoft.com/ja-jp/windows-hardware/drivers/usbcon/usb-3-0-lpm-mechanism-)

## Wake Up Events

[マイクロソフトの説明](https://docs.microsoft.com/ja-jp/windows/win32/power/system-wake-up-events)

## Operators

### UsbCmdRegister

### UsbStatusRegister(USBSTS)

説明 ｜値
-----|----
Address | Operational Base + 04H
Default | 0000 0001
Size | 32Bits

#### HcHalted(0)

R/Oビットが1のときは常に0に設定されます。
R/Oビットが0になったら1になりますが、若干時価感がかかるので待機する必要があります。

#### Rsvdz(1)

予約

#### HostSystemError(2)

ホストシステムのアクセスに失敗したとき1がセットされます。

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

## Device Context

- [slot context](#slot-context)

## Slot Context

スロットコンテキストデータストラクタは、デバイス全体に適用される情報を定義します。

Bits  | 名前　| 説明
------|------|----
19:0  | RouteString |このフィールドは、ハブがパケットを正しいダウンストリームポートにルーティングするために使用されます<br>入力として、このフィールドは、速度に関係なく、すべてのUSBデバイスに対して設定され、USBトポロジ内の位置を示します
23:20 | Speed | 非推奨
24    | 予約領域 |
