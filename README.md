## 概要
負荷試験のためのリポジトリ

## ディレクトリ構成

```bash 
% tree -L 3
.
├── README.md
├── go_test
│   ├── main
│   └── main.go
├── python_test
│   └── main.py
└── rust_test
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
       └── main.rs
```


## サーバーの環境
```bash
% python --version                                                               
Python 3.11.2
 go version                                                                      
% go version 
go1.20.6 darwin/arm64
% rustc --version                                                                
rustc 1.68.2 (9eb3afe9e 2023-03-27) (built from a source tarball)
```

ライブラリのversionは後日記載。

## 負荷試験
今回はPostmanを使用。
今後はk6で試験。
