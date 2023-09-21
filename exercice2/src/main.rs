#![no_std]
#![no_main]
use cortex_m_rt::entry;
use stm32l4xx_hal::{delay::Delay, prelude::*};

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
  let core = cortex_m::Peripherals::take().unwrap();
  let device = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

  let mut flash = device.FLASH.constrain();
  let mut rcc = device.RCC.constrain();
  let mut pwr = device.PWR.constrain(&mut rcc.apb1r1);

  let clocks = rcc.cfgr.sysclk(64.MHz()).pclk1(48.MHz()).freeze(&mut flash.acr, &mut pwr);

  let mut gpioa = device.GPIOA.split(&mut rcc.ahb2);
  let mut gpiob = device.GPIOB.split(&mut rcc.ahb2);

  let mut led1 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

  let mut led2 = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

  let mut timer = Delay::new(core.SYST, clocks);

  println!("Hello, world!");

  led1.set_low();
  led2.set_high();

  loop {
    led1.toggle();
    led2.toggle();
    println!("toggle leds");
    timer.delay_ms(1000_u32);
  }
}
