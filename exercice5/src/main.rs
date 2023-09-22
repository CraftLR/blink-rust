#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
  let p = embassy_stm32::init(Default::default());
  info!("Hello World!");

  let mut led1 = Output::new(p.PA5, Level::High, Speed::Low);
  let mut led2 = Output::new(p.PB14, Level::Low, Speed::Low);

  loop {
    led1.toggle();
    led2.toggle();
    info!("toggle leds");
    Timer::after(Duration::from_millis(1000)).await;
  }
}
