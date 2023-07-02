# thi-monitor-rs

Temperature humidity index monitor to LED

Rustで作る温湿度LEDモニター

## Used package

- avr-gcc
- avr-libc
- avrdude

## Environment
- ArchLinux 6.3.9-arch1-1 x86_64

- rustc 1.68.0-nightly (37d7de337 2022-12-12)
- cargo 1.68.0-nightly (70898e522 2022-12-05)
- ravedude v0.1.5 no git

以下のコマンドを実行してツールチェインのバージョンを揃えてください。
Please run this command.

```sh
rustup override set nightly-2022-12-12
```
```sh
rustup component add rust-src --toolchain nightly-2022-12-12
```
## Run

```sh
cargo run --release
```

## Othrs
- Rust-Analyzer(VSCode)で`#[arduino_hal::entry]`に赤波線が引かれる
    - jsonファイルに`"rust-analyzer.diagnostics.disabled": ["unresolved-proc-macro"]`を追加

## Future
- [x] 温度と湿度を取得してシリアルコンソールに表示
- [ ] LEDのテスト
- [ ] 不快指数を計算してLEDで表現
- [ ] 申し訳程度の抽象化

## Licence
未定(たぶんMITになりそう)
