use embedded_hal::spi::Operation;
use esp_idf_hal::{
    delay::FreeRtos,
    prelude::Peripherals,
    spi::{self, SpiDeviceDriver},
    units,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

// Second experiment controlling the SPI with the library
// Broken because Sparkfun, cf. the comments on the Button board's product page
fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let sck = peripherals.pins.gpio8;
    let cs = peripherals.pins.gpio21;
    let miso = peripherals.pins.gpio20;
    let mosi = peripherals.pins.gpio9;
    let mut spi = SpiDeviceDriver::new_single(
        peripherals.spi2,
        sck,
        miso,
        Some(mosi),
        Some(cs),
        &spi::SpiDriverConfig::new(),
        &spi::SpiConfig::new()
            .data_mode(embedded_hal::spi::MODE_3)
            .bit_order(spi::config::BitOrder::LsbFirst)
            .cs_active_high()
            .baudrate(units::KiloHertz(40).into())
            .input_delay_ns(10 * 1000),
    )?;
    info!("Initialised everything");
    let low_frame = [0u8; 16];
    let up_frame = [255u8; 16];
    let mut read_buffer = [0u8; 16];

    FreeRtos::delay_ms(1000);
    loop {
        spi.transaction(&mut [
            Operation::Write(&up_frame),
            Operation::Write(&low_frame),
            Operation::Write(&up_frame),
            Operation::Read(&mut read_buffer),
        ])?;

        info!("Spi: read {read_buffer:x?}");
        FreeRtos::delay_ms(1000);
        info!("Looped");
    }
}
