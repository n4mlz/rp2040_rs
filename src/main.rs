//! XIAO RP2040 LED点滅の例
//!
//! https://tutoduino.fr/
//!
//! Seeed Studio XIAO RP2040ボード上のLEDを点滅させます。
//!
#![no_std]
#![no_main]
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use seeeduino_xiao_rp2040::entry;
use seeeduino_xiao_rp2040::hal;
use seeeduino_xiao_rp2040::hal::pac;
use seeeduino_xiao_rp2040::hal::prelude::*;
/// ベアメタルアプリケーションへのエントリーポイント。
///
/// `#[entry]`マクロは、グローバル変数の初期化が完了したら、Cortex-Mのスタートアップコードがこの関数を呼び出すようにします。
///
/// 関数はRP2040の周辺機器を設定し、その後無限ループでLEDを点滅させます。
#[entry]
fn main() -> ! {
    // シングルトンオブジェクトを取得します
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    // ウォッチドライバをセットアップします - クロック設定コードに必要です
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    // クロックを設定します
    //
    // デフォルトでは、125 MHzのシステムクロックを生成します
    let clocks = hal::clocks::init_clocks_and_plls(
        seeeduino_xiao_rp2040::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    // シングルサイクルI/OブロックがGPIOピンを制御します
    let sio = hal::Sio::new(pac.SIO);
    // この特定のボードでの機能に従ってピンを設定します
    let pins = seeeduino_xiao_rp2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    // delayオブジェクトを使って指定された時間（ミリ秒単位）待機します
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    // ピンをプッシュプル出力として動作させるように設定します
    let mut led_blue_pin = pins.led_blue.into_push_pull_output();
    let mut led_green_pin = pins.led_green.into_push_pull_output();
    let mut led_red_pin = pins.led_red.into_push_pull_output();
    // 緑と赤のLEDをオフに設定します
    led_green_pin.set_high().unwrap();
    led_red_pin.set_high().unwrap();
    loop {
        // 青のLEDを1秒間点灯させます
        led_blue_pin.set_low().unwrap();
        delay.delay_ms(1000);
        // 青のLEDを500ミリ秒間消灯します
        led_blue_pin.set_high().unwrap();
        delay.delay_ms(500);
    }
}
