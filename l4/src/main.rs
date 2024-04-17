#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, BufferedUart},
    Config,
};
use embedded_io::{Write, Read};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART2 => usart::BufferedInterruptHandler<peripherals::USART2>;
});

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
    let mut tx_buffer = [0; 128];
    let mut rx_buffer = [0; 128];
    let mut usart = unwrap!(BufferedUart::new(
        p.USART2,
        Irqs,
        p.PA3,
        p.PA2,
        &mut tx_buffer,
        &mut rx_buffer,
        config
    ));

    unwrap!(usart.write_all(b"Hello Embassy World!\r\n"));
    info!("wrote Hello, starting echo");

    let mut buf = [0u8; 1];
    loop {
        unwrap!(usart.read_exact(&mut buf));
        unwrap!(usart.write_all(&buf));
    }
}
