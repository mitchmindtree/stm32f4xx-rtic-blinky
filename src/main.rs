#![no_main]
#![no_std]

use panic_halt as _;

use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use rtic::app;
use rtic::cyccnt::U32Ext;
use stm32f4xx_hal::gpio::{gpiod::PD13, Output, PushPull};
use stm32f4xx_hal::prelude::*;

const CYCLE_HZ: u32 = 168_000_000;

#[app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: PD13<Output<PushPull>>,
    }

    #[init(schedule = [blink])]
    fn init(cx: init::Context) -> init::LateResources {
        // Enable cycle counter
        let mut core = cx.core;
        core.DWT.enable_cycle_counter();

        let device: stm32f4xx_hal::stm32::Peripherals = cx.device;

        // Setup clocks
        let rcc = device.RCC.constrain();
        let _clocks = rcc.cfgr.sysclk(CYCLE_HZ.hz()).freeze();

        // Setup LED
        let gpiod = device.GPIOD.split();
        let mut led = gpiod.pd13.into_push_pull_output();
        led.set_low().unwrap();

        // Schedule the blinking task
        cx.schedule.blink(cx.start + CYCLE_HZ.cycles()).unwrap();

        init::LateResources { led }
    }

    #[task(resources = [led], schedule = [blink])]
    fn blink(cx: blink::Context) {
        cx.resources.led.toggle().unwrap();
        cx.schedule.blink(cx.scheduled + CYCLE_HZ.cycles()).unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};
