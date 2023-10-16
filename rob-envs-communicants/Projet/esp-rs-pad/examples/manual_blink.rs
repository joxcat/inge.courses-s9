use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver, prelude::Peripherals};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

// First experiment controlling the SPI manually
fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut sck = PinDriver::output(peripherals.pins.gpio8)?;
    let mut cs = PinDriver::output(peripherals.pins.gpio21)?;
    let mut miso = PinDriver::output(peripherals.pins.gpio20)?;
    let mut led = 0u8;

    info!("Initialised everything");
    FreeRtos::delay_ms(1000);
    loop {
        sck.set_high()?;
        cs.set_high()?;
        FreeRtos::delay_us(15);

        led = !led;

        for _ in 0..4 {
            for _ in 0..128 {
                sck.set_low()?;
                FreeRtos::delay_us(5);

                if led == 0 {
                    miso.set_low()?;
                } else {
                    miso.set_high()?;
                }
                FreeRtos::delay_us(5);

                sck.set_high()?;
                FreeRtos::delay_us(10);
            }
        }

        cs.set_low()?;

        FreeRtos::delay_ms(1000);
        info!("Looped");
    }
}
