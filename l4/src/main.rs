#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::{usart::Uart, Config};
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello World!");

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = true;
        config.rcc.pll = Some(Pll {
            source: PllSource::HSI, // 16 MHz
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL10, // 16 * 10 = 160 MHz
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2), // 160 / 2 = 80 MHz
        });
        config.rcc.sys = Sysclk::PLL1_R;
    }

    let p = embassy_stm32::init(config);

    let config = embassy_stm32::usart::Config::default();
    let mut usart = unwrap!(Uart::new_blocking(p.USART2, p.PA3, p.PA2, config));

    unwrap!(usart.blocking_write(b"Hello Embassy World!\r\n"));
    info!("wrote Hello, starting echo");

    let mut buf = [0u8; 1];
    loop {
        if let Err(e) = usart.blocking_read(&mut buf) {
            defmt::error!("Uart read error: {}", e);
            continue;
        }
        unwrap!(usart.blocking_write(&buf));
    }
}
