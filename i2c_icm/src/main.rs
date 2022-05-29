//! I2c Sample
//!
//! pc0 is i2c3.scl
//! pc1 is i2c3.sda
//!
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_semihosting;

extern crate stm32l4xx_hal as hal;

use crate::hal::i2c;
use crate::hal::i2c::I2c;
use crate::hal::serial::Serial;
use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::rt::ExceptionFrame;
use core::fmt::Write;

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
        .sysclk(80.MHz())
        .pclk1(80.MHz())
        .pclk2(80.MHz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    // The Serial API is highly generic
    // TRY the commented out, different pin configurations
    let tx = gpioa.pa9.into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    // let tx = gpioa.pa2.into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    let rx = gpioa.pa10.into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    // let rx = gpioa.pa3.into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    // TRY using a different USART peripheral here
    let serial = Serial::usart1(dp.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
    let (mut tx, mut rx) = serial.split();
    rx.check_for_error().ok();

    // core::fmt::Write is implemented for tx.
    writeln!(tx, "Hello, world!\r\n").unwrap();

    // I2C GPIO config
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);
    let mut scl = gpioc
        .pc0
        .into_alternate_open_drain(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl);
    scl.internal_pull_up(&mut gpioc.pupdr, true);

    let mut sda = gpioc
        .pc1
        .into_alternate_open_drain(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl);
    sda.internal_pull_up(&mut gpioc.pupdr, true);

    // I2C3 init
    let mut i2c = I2c::i2c3(
        dp.I2C3,
        (scl, sda),
        i2c::Config::new(100.kHz(), clocks),
        &mut rcc.apb1r1,
    );

    // I2C3 read ICM20608 reg.
    const ICM20608_ADDR: u8 = 0x68;
    const ICM20608D_WHO_AM_I: u8 = 0xAE;
    let mut buffer = [0u8; 1];

    i2c.write_read(ICM20608_ADDR, &[0x75], &mut buffer).unwrap();
    let id: u8 = buffer[0];
    
    writeln!(tx, "i2c20608 id: 0x{:X}\r\n", id).ok();
    if id == ICM20608D_WHO_AM_I {
        writeln!(tx, "i2c20608 read id OK!\r\n").ok();
    }

    // Get the delay provider.
    let mut timer = Delay::new(cp.SYST, clocks);
    
    loop {

        timer.delay_ms(500 as u32);
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
