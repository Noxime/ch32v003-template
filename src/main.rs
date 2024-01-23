#![no_std]
#![no_main]

use panic_halt as _;
use ch32v00x_hal::{self as hal, prelude::*};

#[ch32v_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    
    let mut rcc = p.RCC.constrain();
    let c = p.GPIOC.split(&mut rcc);

    c.pc1.into_push_pull_output().set_high();
    c.pc2.into_push_pull_output().set_low();

    loop {}
}
