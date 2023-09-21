#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::{entry, exception, ExceptionFrame};

use stm32l4xx_hal::{
  delay::Delay,
  gpio::{gpioc::PC13, gpioc::PC9, Edge, Input, Output, PullUp, PushPull},
  interrupt,
  prelude::*,
};

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

type Button = PC13<Input<PullUp>>;
static USER_BUTTON: Mutex<RefCell<Option<Button>>> = Mutex::new(RefCell::new(None));

type LedPin = PC9<Output<PushPull>>;
static LED3: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
  let core = cortex_m::Peripherals::take().unwrap();
  let mut device = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

  let mut flash = device.FLASH.constrain();
  let mut rcc = device.RCC.constrain();
  let mut pwr = device.PWR.constrain(&mut rcc.apb1r1);

  let clocks = rcc.cfgr.sysclk(64.MHz()).pclk1(48.MHz()).freeze(&mut flash.acr, &mut pwr);

  let mut gpioa = device.GPIOA.split(&mut rcc.ahb2);
  let mut gpiob = device.GPIOB.split(&mut rcc.ahb2);
  let mut gpioc = device.GPIOC.split(&mut rcc.ahb2);

  let mut led1 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

  let mut led2 = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

  let mut led3 = gpioc.pc9.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

  led3.set_low();

  let mut button = gpioc.pc13.into_pull_up_input(&mut gpioc.moder, &mut gpioc.pupdr);

  let mut timer = Delay::new(core.SYST, clocks);

  button.make_interrupt_source(&mut device.SYSCFG, &mut rcc.apb2);
  button.enable_interrupt(&mut device.EXTI);
  button.trigger_on_edge(&mut device.EXTI, Edge::Rising);

  cortex_m::interrupt::free(|cs| {
    USER_BUTTON.borrow(cs).replace(Some(button));
    LED3.borrow(cs).replace(Some(led3));
  });

  unsafe {
    cortex_m::peripheral::NVIC::unmask(stm32l4xx_hal::interrupt::EXTI15_10);
  }

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

// define the hard fault handler
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
  core::panic!("{:#?}", ef);
}

// define the default exception handler
#[exception]
unsafe fn DefaultHandler(irqn: i16) -> ! {
  core::panic!("unhandled exception (IRQn={irqn})");
}

#[interrupt]
fn EXTI15_10() {
  cortex_m::interrupt::free(|cs| {
    USER_BUTTON.borrow(cs).borrow_mut().as_mut().unwrap().clear_interrupt_pending_bit();
    LED3.borrow(cs).borrow_mut().as_mut().unwrap().toggle();
  });
  println!("button pushed !");
}
