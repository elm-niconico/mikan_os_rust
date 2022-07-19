# 目次

- [目次](#目次)
  - [命名規則](#命名規則)
  - [生ポインタ](#生ポインタ)
  - [build-stdフラグ](#build-stdフラグ)
  - [参考](#参考)
    - [お気楽Rustプログラミング超入門](#お気楽rustプログラミング超入門)

## 命名規則

[このページ](https://sinkuu.github.io/api-guidelines/naming.html)参照

特に変換メソッドの命名規則は標準ライブラリなどでも多用されているため、覚えておいたほうがよさそう
プレフィクス | コスト | 所有権
-------|-----|----
as_ | 低い | 借用->借用
to_ | 高い | 借用->所有(Copyでない型)
to_ | 高い | 所有->所有(Copy型)
into_ | 可変 | 所有->所有(Copyでない型)

## 生ポインタ

---

[お気楽Rustプログラミング超入門](#http://www.nct9.ne.jp/m_hiroi/linux/rustabc04.html)が参考になった。

メソッド定義については[公式リファレンス](https://doc.rust-lang.org/std/primitive.pointer.html)参照

## build-stdフラグ

---
標準ライブラリを一緒にコンパイルできるようになる機能  
cargoのnightly版が必要  
ビルドターゲットの指定も必要  
[公式リファレンス](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std)

## 参考

---

### お気楽Rustプログラミング超入門

ポインタの基本、マクロの基本、ライフタイム境界の説明などが記されている  
<http://www.nct9.ne.jp/m_hiroi/linux/rustabc04.html>
