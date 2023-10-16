use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

/// Implementation based on <https://www.sparkfun.com/datasheets/Widgets/ButtonPadControllerSPI_UserGuide_v2.pdf>
/// and <https://hblok.net/blog/posts/2010/10/17/button_pad_controller_spi/>
pub struct ButtonPadController<'i, PinSck, PinCs, PinMiso, PinMosi>
where
    PinSck: esp_idf_hal::gpio::Pin,
    PinCs: esp_idf_hal::gpio::Pin,
    PinMiso: esp_idf_hal::gpio::Pin,
    PinMosi: esp_idf_hal::gpio::Pin,
{
    red: [u8; 16],
    green: [u8; 16],
    blue: [u8; 16],
    buttons: [bool; 16],
    sck: PinDriver<'i, PinSck, esp_idf_hal::gpio::Output>,
    cs: PinDriver<'i, PinCs, esp_idf_hal::gpio::Output>,
    miso: PinDriver<'i, PinMiso, esp_idf_hal::gpio::Output>,
    mosi: PinDriver<'i, PinMosi, esp_idf_hal::gpio::Input>,
}

#[allow(unused)]
impl<'i, PinSck, PinCs, PinMiso, PinMosi> ButtonPadController<'i, PinSck, PinCs, PinMiso, PinMosi>
where
    PinSck: esp_idf_hal::gpio::Pin + esp_idf_hal::gpio::OutputPin,
    PinCs: esp_idf_hal::gpio::Pin + esp_idf_hal::gpio::OutputPin,
    PinMiso: esp_idf_hal::gpio::Pin + esp_idf_hal::gpio::OutputPin,
    PinMosi: esp_idf_hal::gpio::Pin + esp_idf_hal::gpio::InputPin,
{
    /// Read only view of the buttons state
    pub fn buttons(&self) -> [bool; 16] {
        self.buttons
    }

    /// Initialize the ButtonPadController
    pub fn new(sck: PinSck, cs: PinCs, miso: PinMiso, mosi: PinMosi) -> anyhow::Result<Self> {
        Ok(Self {
            red: [0; 16],
            green: [0; 16],
            blue: [0; 16],
            buttons: [false; 16],
            sck: PinDriver::output(sck)?,
            cs: PinDriver::output(cs)?,
            miso: PinDriver::output(miso)?,
            mosi: PinDriver::input(mosi)?,
        })
    }

    /// Write (only) frame on the SPI bus
    fn write_frame(&mut self, frame: &[u8; 16]) -> anyhow::Result<()> {
        for bit in bit_vec::BitVec::from_bytes(frame).iter().rev() {
            self.sck.set_low()?;
            FreeRtos::delay_us(5);

            trace!("Writing bit on miso");
            if bit {
                self.miso.set_high()?;
            } else {
                self.miso.set_low()?;
            }
            FreeRtos::delay_us(5);

            self.sck.set_high()?;
            FreeRtos::delay_us(10);
        }
        Ok(())
    }
    /// Transfare frame (write a dummy one and read) on the SPI bus
    fn transfer_frame(&mut self, frame_buffer: &mut [u8; 16]) -> anyhow::Result<()> {
        let mut bits = bit_vec::BitVec::from_elem(128, false);

        for i in 0..bits.len() {
            self.sck.set_low()?;
            FreeRtos::delay_us(5);

            trace!("Writing bit on miso");
            self.miso.set_high()?;
            FreeRtos::delay_us(5);

            self.sck.set_high()?;
            FreeRtos::delay_us(5);

            trace!("Reading bit on mosi");
            bits.set(i, self.mosi.is_high());
            FreeRtos::delay_us(10);
        }

        let mut bytes = bits.to_bytes();
        bytes.reverse();
        frame_buffer.copy_from_slice(&bytes);

        Ok(())
    }
    /// Send the state on the button board
    fn commit(&mut self) -> anyhow::Result<()> {
        let red_buffer = self.red;
        let green_buffer = self.green;
        let blue_buffer = self.blue;

        self.sck.set_high()?;
        debug!("Locking SPI");
        self.cs.set_high()?;
        FreeRtos::delay_us(15);

        self.write_frame(&red_buffer)?;
        self.write_frame(&green_buffer)?;
        self.write_frame(&blue_buffer)?;

        let mut buttons_buffer = [0u8; 16];
        self.transfer_frame(&mut buttons_buffer);
        self.buttons = buttons_buffer.map(|byte| byte == 0x00);
        debug!("Buttons state is {:x?}", self.buttons);

        self.cs.set_low()?;
        debug!("Unlocking SPI");
        FreeRtos::delay_us(500);
        Ok(())
    }
    // Simple actions on the leds
    pub fn blackout(&mut self) -> anyhow::Result<()> {
        self.red = [0; 16];
        self.green = [0; 16];
        self.blue = [0; 16];
        self.commit()?;
        Ok(())
    }
    pub fn lightup(&mut self) -> anyhow::Result<()> {
        self.red = [255; 16];
        self.green = [255; 16];
        self.blue = [255; 16];
        self.commit()?;
        Ok(())
    }
    pub fn red(&mut self) -> anyhow::Result<()> {
        self.red = [255; 16];
        self.green = [0; 16];
        self.blue = [0; 16];
        self.commit()?;
        Ok(())
    }
    pub fn green(&mut self) -> anyhow::Result<()> {
        self.red = [0; 16];
        self.green = [255; 16];
        self.blue = [0; 16];
        self.commit()?;
        Ok(())
    }
    pub fn blue(&mut self) -> anyhow::Result<()> {
        self.red = [0; 16];
        self.green = [0; 16];
        self.blue = [255; 16];
        self.commit()?;
        Ok(())
    }
    /// Write a hex RGB matrix on the board
    pub fn write_matrix(&mut self, matrix: &[u32; 16]) -> anyhow::Result<()> {
        self.red = matrix.map(|num| ((num >> 16) & 255) as u8);
        debug!("red {:x?}", self.red);
        self.green = matrix.map(|num| ((num >> 8) & 255) as u8);
        debug!("green {:x?}", self.green);
        self.blue = matrix.map(|num| (num & 255) as u8);
        debug!("blue {:x?}", self.blue);
        self.commit()?;
        Ok(())
    }
}
