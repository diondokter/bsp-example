#![no_std]

use defmt::{info, unwrap, Format};

pub fn uart_echo<Uart>(mut uart: Uart) -> !
where
    Uart: embedded_io::Read + embedded_io::Write,
    Uart::Error: Format,
{
    unwrap!(uart.write_all(b"Hello Embassy World!\r\n"));
    info!("wrote Hello, starting echo");

    let mut buf = [0];
    loop {
        unwrap!(uart.read_exact(&mut buf));
        unwrap!(uart.write_all(&mut buf));
    }
}
