#![no_main]
#![no_std]

use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::uwriteln;
use ufmt_float::uFmt_f32;

// use i2c sensor address
// Write direction test:
//
// -    0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
// 00:       -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 30: -- -- -- -- -- -- -- -- 38 -- -- -- -- -- -- --
// 40: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 70: -- -- -- -- -- -- -- --

// Read direction test
// -    0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
// 00:       -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 30: -- -- -- -- -- -- -- -- 38^C

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    let mut check = [0x71u8];
    let mut dat = [0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8];
    let mut trigger = [0xACu8, 0x33u8, 0x00u8];

    let mut _led = pins.d13.into_output();

    // 初期チェック
    arduino_hal::delay_ms(100);
    let _ret: Result<(), arduino_hal::i2c::Error> = i2c.read(0x38, &mut check);

    loop {
        // データ書き込み
        arduino_hal::delay_ms(100);
        let _result: Result<(), arduino_hal::i2c::Error> = i2c.write(0x38u8, &mut trigger);

        // データ読み取り
        arduino_hal::delay_ms(800);
        let _result: Result<(), arduino_hal::i2c::Error> = i2c.read(0x38, &mut dat);

        // 返却データを変換
        let hum: u32 =
            ((dat[1] as u32) << 12 | (dat[2] as u32) << 4 | ((dat[3] as u32 & 0xF0) >> 4)).into();
        let tmp: u32 =
            (((dat[3] as u32 & 0x0F) << 16) | (dat[4] as u32) << 8u8 | dat[5] as u32).into();

        // 温度と湿度に変換
        let calced_tmp = (tmp as f32 / 1048576.0) * 200.0 - 50.0;
        let calced_hum = (hum as f32 / 1048576.0) * 100.0;

        // フォーマット
        let fmted_tmp = uFmt_f32::Zero(calced_tmp);
        let fmted_hum = uFmt_f32::Zero(calced_hum);

        // error
        uwriteln!(&mut serial, "===================").void_unwrap();
        uwriteln!(&mut serial, "室温 {}℃", fmted_tmp).void_unwrap();
        uwriteln!(&mut serial, "湿度 {}%", fmted_hum).void_unwrap();
        uwriteln!(&mut serial, "===================").void_unwrap();

        arduino_hal::delay_ms(1000);
    }
}

// fn flip(x: f32) -> u32 {
//     let y: u32 = x.to_bits();
//     return y ^ ((-((y >> 31) as i32) as u32 | 0x80000000_u32) as u32);
// }
// 695744
// 375908

// 2^20=1048576
