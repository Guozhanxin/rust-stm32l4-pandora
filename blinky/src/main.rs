//! blinky Sample
//!
//! pe7 is used as pandora LED.
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

#[entry]
fn main() -> ! {

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
    let mut led = gpioe
        .pe7
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    // Get the delay provider.
    let mut timer = Delay::new(cp.SYST, clocks);

    loop {
        led.set_low().ok();
        timer.delay_ms(1000 as u32);

        led.set_high().ok();
        timer.delay_ms(1000 as u32);
    }

}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}