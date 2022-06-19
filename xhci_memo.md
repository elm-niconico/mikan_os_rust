# 目次

- [目次](#目次)
  - [USBリンク](#usbリンク)
  - [Wake Up Events](#wake-up-events)
  - [Runtime Registers](#runtime-registers)
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

## Runtime Registers

このレジスタスペースのベースアドレスは、ランタイムベースと呼ばれます。  
ランタイムベースは32Byteにアラインされていなければならず、Capability Base Addressのオフセットの値を追加することによって計算されます。


## Device Context

- [slot context](#slot-context)

## Slot Context

スロットコンテキストデータストラクタは、デバイス全体に適用される情報を定義します。

Bits  | 名前　| 説明
------|------|----
19:0  | RouteString |このフィールドは、ハブがパケットを正しいダウンストリームポートにルーティングするために使用されます<br>入力として、このフィールドは、速度に関係なく、すべてのUSBデバイスに対して設定され、USBトポロジ内の位置を示します
23:20 | Speed | 非推奨
24    | 予約領域 |
