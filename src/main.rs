#![no_std]
#![no_main]

use panic_halt as _;
use nrf_softdevice as _;
use defmt_rtt as _;

#[cortex_m_rt::entry]
fn main() -> ! {

    let mut i = 0;
defmt::println!("Start::::..");
    loop {
        defmt::println!("Loop {}...", i);

        i += 1;
    }
}