#![no_main]
#![no_std]

use arduino_hal::{port::mode::Output, prelude::*};
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

    const LED_PIN_RED: u8 = 9;
    const LED_PIN_GRN: u8 = 10;
    const LED_PIN_BLE: u8 = 11;

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    let mut red_led = pins.d9.into_output();
    let mut grn_led = pins.d10.into_output();
    let mut ble_led = pins.d11.into_output();

    let mut check = [0x71u8];
    let mut dat = [0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8];
    let mut trigger = [0xACu8, 0x33u8, 0x00u8];

    // 初期チェック
    arduino_hal::delay_ms(100);
    let _ret: Result<(), arduino_hal::i2c::Error> = i2c.read(0x38, &mut check);
    red_led.set_low();
    ble_led.set_low();
    grn_led.set_low();
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
        // 不快指数計算
        let calced_thi = 0.81 * calced_tmp + 0.01 * calced_hum * (0.99 * calced_tmp - 14.3) + 46.3;
        let calced_thi = calced_thi as u32;
        // フォーマット
        let fmted_tmp = uFmt_f32::Zero(calced_tmp);
        let fmted_hum = uFmt_f32::Zero(calced_hum);
        // let fmted_thi = uFmt_f32::Zero(calced_thi);

        // 一旦表示
        uwriteln!(&mut serial, "===================").void_unwrap();
        uwriteln!(&mut serial, "室温 {}℃", fmted_tmp).void_unwrap();
        uwriteln!(&mut serial, "湿度 {}%", fmted_hum).void_unwrap();
        uwriteln!(&mut serial, "不快指数 {}", calced_thi).void_unwrap();
        uwriteln!(&mut serial, "===================").void_unwrap();

        // ~50 青点滅
        // 50~55 青点灯
        // 55~60 水色
        // 60~65 白
        // 65~70 緑 快適
        // 70~75 黄色
        // 75~80 オレンジ
        // 80~85　赤
        // 85~ 赤点滅

        match calced_thi {
            0...49 => {
                if red_led.is_set_high() {
                    red_led.set_low();
                }
                if grn_led.is_set_high() {
                    grn_led.set_low();
                }
                ble_led.toggle();
            }
            50...54 => {
                if red_led.is_set_high() {
                    red_led.set_low();
                }
                if grn_led.is_set_high() {
                    grn_led.set_low();
                }
                ble_led.set_high();
            }
            55...59 => {
                if red_led.is_set_high() {
                    red_led.set_low();
                }
                grn_led.set_high();
                ble_led.set_high();
            }
            60...64 => {
                red_led.set_high();
                grn_led.set_high();
                ble_led.set_high();
            }
            65...69 => {
                if red_led.is_set_high() {
                    red_led.set_low();
                }
                if ble_led.is_set_high() {
                    ble_led.set_low();
                }
                grn_led.set_high();
            }
            70...74 => {
                if red_led.is_set_high() != grn_led.is_set_high() {
                    red_led.set_high();
                    grn_led.set_high();
                }
                if ble_led.is_set_high() {
                    ble_led.set_low();
                }
                red_led.set_high();
                grn_led.set_high();
            }
            75...79 => {
                if red_led.is_set_high() != grn_led.is_set_high() {
                    red_led.set_high();
                    grn_led.set_high();
                }
                if ble_led.is_set_high() {
                    ble_led.set_low();
                }
                red_led.toggle();
                grn_led.toggle();
            }
            80...84 => {
                red_led.set_high();
                grn_led.set_low();
                ble_led.set_low();
            }
            85...100 => {
                if grn_led.is_set_high() {
                    grn_led.set_low();
                }
                if ble_led.is_set_high() {
                    ble_led.set_low();
                }
                red_led.toggle();
            }
            _ => {
                red_led.toggle();
                grn_led.set_low();
                ble_led.set_low();
            }
        }
    }
}

// fn flip(x: f32) -> u32 {
//     let y: u32 = x.to_bits();
//     return y ^ ((-((y >> 31) as i32) as u32 | 0x80000000_u32) as u32);
// }
// 695744
// 375908

// 2^20=1048576
