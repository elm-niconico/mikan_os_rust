# 目次

- [目次](#目次)
  - [3章](#3章)

## 3章

kernel.elfを作成したが、エントリーポイントのアドレスが書籍とずれている
[MikanOsのGitのIssues41](https://github.com/uchan-nos/os-from-zero/issues/41)で自分と同じ症状の人が質問しており、そこではld.lldのバージョンに問題があるかもしれないと記載されていた。  
lldのバージョンを9にダウン、また、[-z separate-code]のコマンドも追加したが、やはり駄目だった。  
4章でブートローダーからカーネルのエントリーポイントを動的に検索する処理を実装することだったため、いったん保留にする。