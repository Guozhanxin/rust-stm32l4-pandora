//! key sample
//!
//! key0 use PD10, RGB_R will light when key0 pull down.
//! 
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
// #[macro_use(block)]
extern crate nb;
extern crate panic_semihosting;

extern crate stm32l4xx_hal as hal;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::hal::serial::{Config, Serial};
use crate::rt::ExceptionFrame;

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (clocks run at nearly the maximum frequency)
    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    // The Serial API is highly generic
    // TRY the commented out, different pin configurations
    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    // let tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);

    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    // let rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);

    // TRY using a different USART peripheral here
    let serial = Serial::usart1(
        dp.USART1,
        (tx, rx),
        Config::default().baudrate(115_200.bps()),
        clocks,
        &mut rcc.apb2,
    );
    let (mut tx, mut rx) = serial.split();
    rx.check_for_error().unwrap();

    // core::fmt::Write is implemented for tx.
    writeln!(tx, "Hello, world!").unwrap();

    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb2);
    let key0 = gpiod
        .pd10
        .into_pull_down_input(&mut gpiod.moder, &mut gpiod.pupdr);
        
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb2);
    let mut led = gpioe
        .pe7
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    // Get the delay provider.
    let mut timer = Delay::new(cp.SYST, clocks);

    loop {
        if key0.is_low().unwrap() {
            timer.delay_ms(50 as u32);
            if key0.is_low().unwrap() {
                // Key down
                writeln!(tx, "key down!\r\n").unwrap();
                led.set_low().ok();
            }
        }else {
            led.set_high().ok();
        }
        
        timer.delay_ms(10 as u32);
    }

}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}