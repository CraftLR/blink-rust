#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Input, Level, Output, Pull, Speed},
};

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_s: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led2 = Output::new(p.PB14, Level::Low, Speed::VeryHigh);
    let mut button = ExtiInput::new(Input::new(p.PC13, Pull::Up), p.EXTI13);

    loop {
        button.wait_for_any_edge().await;
        if button.is_low() {
            info!("button pressed");
            led2.set_high();
        } else {
            info!("button released");
            led2.set_low();
        }
    }
}