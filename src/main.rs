#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;
use stm32f4xx_hal::prelude::*;  // Hz()
use stm32f4xx_hal::i2c::*;

const DEVICE_ADRS_FOR_WRITE: u8 = 0xa0;

#[entry]
fn main() -> ! {

    let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();   // GPIOBのclockも有効にしてくれる （AHBENRレジスタ）

    let rcc = dp.RCC.constrain();       // RCCの取得
    let clks = rcc.cfgr.freeze();   // 各clockの設定
    let mode = Mode::Standard { frequency: 100_000_u32.Hz() };

    let mut i2c = I2c::new(
        dp.I2C1,
        (gpiob.pb8, gpiob.pb9),
        mode,
        &clks,
    );  // I2Cの生成

    let wbuf = [0, 1, 18, 52, 86];
    // 0, 1 : 書き込むメモリーの先頭アドレス (0x0001 番地)
    // 18, 52, 86 はメモリーに書く値（数値に意味はない）

    let adrs = DEVICE_ADRS_FOR_WRITE >> 1;  // 中で左シフトされるので右シフトしておく
    let _ = i2c.write(adrs, &wbuf);

    let mut rbuf = [0; 3];

    let _ = i2c.write_read(adrs, &wbuf[..2], &mut rbuf);    // 第2引数はスライスを使って最初の２バイトのみ書くように指示する

    loop {
    }
}
