# thi-monitor-rs

Temperature humidity index monitor to LED

Rustで作る温湿度LEDモニター

## Used package

- avr-gcc
- avr-libc
- avrdude


## Hardware Environment

- Arduino UNO R3
- RGP LED (カソードコモン)
- KP-DHT20 温湿度センサ
- お使いのコンピュータ

## Software Environment
### OS
- ArchLinux 6.3.9-arch1-1 x86_64

### Build test
|:-------:|:-----:|:---------:|
| Windows | macOS | GNU/Linux |
|  None   | None  |  __OK!__  |

- issue求む

### Tools

- rustc 1.68.0-nightly (37d7de337 2022-12-12)
- cargo 1.68.0-nightly (70898e522 2022-12-05)
- ravedude v0.1.5 no git
- avr-gcc
- avr-libc
- avrdude

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

- `--release`をつけないとavr-gccがエラーを吐きます。

## Config
- Rust-Analyzer(VSCode)で`#[arduino_hal::entry]`に赤波線が引かれる
    - Rust-Analyzerの設定jsonファイルに`"rust-analyzer.diagnostics.disabled": ["unresolved-proc-macro"]`を追加

## Future
- [x] 温度と湿度を取得してシリアルコンソールに表示
- [ ] LEDのテスト
- [ ] 不快指数を計算してLEDで表現
- [ ] 申し訳程度の抽象化

## License
`MIT License`と`Apache License v2.0`とのデュアルライセンスです。
