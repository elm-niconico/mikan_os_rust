# セグメント

## 目次

- [セグメント](#セグメント)
  - [目次](#目次)
  - [概要](#概要)
    - [もともとの機能](#もともとの機能)
  - [64BitModeでの役割](#64bitmodeでの役割)
  - [参考](#参考)
    - [x86_64アーキテクチャ - ばびろん](#x86_64アーキテクチャ---ばびろん)

## 概要

### もともとの機能

メモリを区画ごとに分ける役割があったようですが、x86_64の64Bitではページングがその役割を担うようになり、事実上無効化されたようです。

## 64BitModeでの役割

ゼロからのOS自作入門より引用
> CPUの動作権限を決定するための機能といえます。
> CPUは現在の動作権限(0-3)を持っていて、その権限で実行しています。

## 参考

### x86_64アーキテクチャ - ばびろん

<https://babyron64.hatenablog.com/entry/2017/12/22/210124#fn:1>
