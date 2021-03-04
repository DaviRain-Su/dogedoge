# dogedoge

[![Build Status](https://travis-ci.org/DaviRain-Su/dogedoge.svg?branch=main)](https://travis-ci.org/DaviRain-Su/dogedoge)

## Build

默认是编译debug模式
```shell
    cargo build 
```
编译release模式
```shell
    cargo build --release
```

## Run

debug模式打开，能查看中间输出的内容
```shell
    RUST_LOG=debug cargo run
```
或者不加直接运行
```shell
    cargo run
```