//! RGB Sample
//!
//! pe7 is used as pandora LED_R.
//! pe8 is used as pandora LED_B.
//! pe9 is used as pandora LED_G.
//!
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_semihosting;

extern crate stm32l4xx_hal as hal;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::rt::ExceptionFrame;

static LED_ON:i32 = 1;
static LED_OFF:i32 = 0;
/* 定义 8 组 LED 闪灯表，其顺序为 R G B */
static _BLINK_TAB: [[i32; 3]; 8] = [
    [LED_ON, LED_ON, LED_ON],
    [LED_OFF, LED_ON, LED_ON],
    [LED_ON, LED_OFF, LED_ON],
    [LED_ON, LED_ON, LED_OFF],
    [LED_OFF, LED_OFF, LED_ON],
    [LED_ON, LED_OFF, LED_OFF],
    [LED_OFF, LED_ON, LED_OFF],
    [LED_OFF, LED_OFF, LED_OFF],
];

#[entry]
fn main() -> ! {
    
    let mut index;

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (clocks run at nearly the maximum frequency)
    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb2);
    let mut led_r = gpioe
        .pe7
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_b = gpioe
    .pe8
    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_g = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    // Get the delay provider.
    let mut timer = Delay::new(cp.SYST, clocks);
    
    index = 0;
    loop {
        let r:i32 = _BLINK_TAB[index][0];
        let g:i32 = _BLINK_TAB[index][1];
        let b:i32 = _BLINK_TAB[index][2];
        if r == LED_ON {
            led_r.set_high().ok();
        }else{
            led_r.set_low().ok();
        }
        if g == LED_ON {
            led_g.set_high().ok();
        }else{
            led_g.set_low().ok();
        }
        if b == LED_ON {
            led_b.set_high().ok();
        }else{
            led_b.set_low().ok();
        }

        index = index + 1;
        if index == 8 {
            index = 0;
        }
        timer.delay_ms(500 as u32);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}