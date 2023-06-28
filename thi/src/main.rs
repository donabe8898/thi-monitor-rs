#![no_main]
#![no_std]

use arduino_hal::{i2c::Direction, prelude::*};
use panic_halt as _;

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

    // ufmt::uwriteln!(&mut serial, "Write direction test:\r").void_unwrap();
    // i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Write);
    // ufmt::uwriteln!(&mut serial, "\r\nRead direction test\r").void_unwrap();
    // i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Read);
    let mut led = pins.d13.into_output();

    // 初期チェック
    arduino_hal::delay_ms(100);
    let ret: Result<(), arduino_hal::i2c::Error> = i2c.read(0x38, &mut check);

    loop {
        arduino_hal::delay_ms(100);
        let result: Result<(), arduino_hal::i2c::Error> = i2c.write(0x38u8, &mut trigger);
        ufmt::uwriteln!(&mut serial, "{:?}", result);

        arduino_hal::delay_ms(800);
        let result: Result<(), arduino_hal::i2c::Error> = i2c.read(0x38, &mut dat);
        ufmt::uwriteln!(&mut serial, "{:?}", result);

        let hum: u32 =
            ((dat[1] as u32) << 12 | (dat[2] as u32) << 4 | ((dat[3] as u32 & 0xF0) >> 4)).into();
        let tmp: u32 =
            (((dat[3] as u32 & 0x0F) << 16) | (dat[4] as u32) << 8u8 | dat[5] as u32).into();

        // ufmt::uwriteln!(&mut serial, "{:?}", hum);
        // ufmt::uwriteln!(&mut serial, "{:?}", tmp);

        // 温度と湿度に変換
        let calced_hum: f64 = (hum as f64 / 1048576.0) * 100.0;
        let calced_tmp: f64 = (tmp as f64 / 104857.0) * 200.0 - 50.0;

        // error
        ufmt::uwriteln!(&mut serial, "{:?}", flip(calced_hum));
        ufmt::uwriteln!(&mut serial, "{:?}", flip(calced_tmp));

        // ufmt::uwriteln!(&mut serial, "{}", calced_hum);
        // ufmt::uwriteln!(&mut serial, "{}", calced_tmp);
        arduino_hal::delay_ms(100);
    }
}

fn flip(x: f64) -> u64 {
    let y: u64 = x.to_bits();
    return y ^ ((-((y >> 63) as i64) as u64 | 0x8000000000000000_u64) as u64);
}
// 695744
// 375908

// 2^20=1048576
