#![no_main]
#![no_std]

use cortex_m_rtic::app;
use stm32f4::stm32f401;

// 入退室検知用のピン設定
const ENTRY_SENSOR_PIN: u16 = 0;
const EXIT_SENSOR_PIN: u16 = 1;

// LEDのピン設定
const LED_PIN: u16 = 13;

#[app(device = stm32f401, peripherals = true)]
const APP: () = {
    struct Resources {
        // 入退室検知用のピン
        entry_sensor: stm32f401::GPIOA,
        exit_sensor: stm32f401::GPIOA,

        // LEDのピン
        led: stm32f401::GPIOC,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // デバイスのパーフェリアルを取得
        let dp = cx.device;

        // 入退室検知用のピンを設定
        let entry_sensor = dp.GPIOA.split();
        let exit_sensor = dp.GPIOA.split();

        // LEDのピンを設定
        let led = dp.GPIOC.split();

        init::LateResources {
            entry_sensor,
            exit_sensor,
            led,
        }
    }

    #[idle(resources = [entry_sensor, exit_sensor, led])]
    fn idle(cx: idle::Context) -> ! {
        // メインループ
        loop {
            // 入室検知
            if cx.resources.entry_sensor.idr.read().bits() & (1 << ENTRY_SENSOR_PIN) != 0 {
                // 入室検知した場合の処理
                cx.resources.led.bsrr.write(|w| w.bs13().set_bit());
            } else {
                // 退室検知した場合の処理
                cx.resources.led.bsrr.write(|w| w.br13().set_bit());
            }
        }
    }
};
