use esp32_nimble::{utilities::BleUuid, uuid128, BLEDevice, NimbleProperties};
use esp_idf_hal::{delay::FreeRtos, gpio, gpio::PinDriver, prelude::Peripherals};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

mod button_pad_controller;

const BLE_DEVICE_NAME: &str = "ESP32-RS ROUILLE";
const BLE_SERVICE_UUID: BleUuid = uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa");
const BLE_PRESSED_BUTTON_CHARACTERISTIC_UUID: BleUuid =
    uuid128!("a3c87500-8ed3-4bdf-8a39-a01bebede295");

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Button Pad Controller setup
    let peripherals = Peripherals::take().unwrap();
    let mut button_pad = button_pad_controller::ButtonPadController::new(
        peripherals.pins.gpio8,
        peripherals.pins.gpio21,
        peripherals.pins.gpio20,
        peripherals.pins.gpio9,
    )?;

    // Additional buttons
    let mut btn_17 = PinDriver::input(peripherals.pins.gpio0)?;
    btn_17.set_pull(gpio::Pull::Down)?;
    let mut btn_18 = PinDriver::input(peripherals.pins.gpio1)?;
    btn_18.set_pull(gpio::Pull::Down)?;
    let mut btn_19 = PinDriver::input(peripherals.pins.gpio2)?;
    btn_19.set_pull(gpio::Pull::Down)?;
    let mut btn_20 = PinDriver::input(peripherals.pins.gpio3)?;
    btn_20.set_pull(gpio::Pull::Down)?;

    /* Bluetooth setup */
    let ble_device = BLEDevice::take();
    let server = ble_device.get_server();
    server.on_connect(|server, desc| {
        info!("New client connected to BLE server");

        server
            .update_conn_params(desc.conn_handle, 24, 48, 0, 60)
            .unwrap();

        info!("Multi-connect support: start advertising");
        ble_device.get_advertising().start().unwrap();
    });
    let service = server.create_service(BLE_SERVICE_UUID);

    let notifying_characteristic = service.lock().create_characteristic(
        BLE_PRESSED_BUTTON_CHARACTERISTIC_UUID,
        NimbleProperties::READ | NimbleProperties::NOTIFY,
    );
    notifying_characteristic.lock().set_value(&[0u8]);

    let ble_advertising = ble_device.get_advertising();
    ble_advertising
        .name(BLE_DEVICE_NAME)
        .add_service_uuid(BLE_SERVICE_UUID);
    ble_advertising.start().unwrap();

    /* Reset of the Button Pad */
    FreeRtos::delay_ms(100);
    button_pad.lightup()?;
    FreeRtos::delay_ms(100);
    button_pad.blackout()?;
    FreeRtos::delay_ms(5);

    info!("Initialised everything, start interacting");
    let mut last_button_pressed: u8 = 0;
    loop {
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(10);

        /* Notify bluetooth on Keypress */
        let mut pressed = false;
        let mut btn_handler = |btn_num| {
            if last_button_pressed == 0 {
                info!("Pressed {btn_num}");
                last_button_pressed = btn_num;
                notifying_characteristic
                    .lock()
                    .set_value(&[last_button_pressed])
                    .notify();
            }
        };

        // Notify for addition buttons
        if btn_17.is_high() {
            pressed = true;
            btn_handler(17);
        } else if btn_18.is_high() {
            pressed = true;
            btn_handler(18);
        } else if btn_19.is_high() {
            pressed = true;
            btn_handler(19);
        } else if btn_20.is_high() {
            pressed = true;
            btn_handler(20);
        }

        // Notify for led matrix
        if !pressed {
            for (idx, button) in button_pad.buttons().into_iter().enumerate() {
                if button {
                    pressed = true;
                    let button_num = (idx as u8) + 1;
                    btn_handler(button_num);
                    break;
                }
            }
        }
        if !pressed && last_button_pressed != 0 {
            // Button released
            debug!("Released button");
            last_button_pressed = 0;
            notifying_characteristic.lock().set_value(&[0u8]).notify();
        }

        /* Update the matrix with the pressed button */
        button_pad.write_matrix(
            /*
            // This could also be a fixed matrix
            0x000000, 0x000000, 0x00dead, 0xbeef00,
            0x000000, 0x336699, 0x000000, 0x000000,
            0x00ffff, 0x000000, 0xffff00, 0x000000,
            0x0000ff, 0x000000, 0x000000, 0x000000,

            // Or a dynamic one like the following
            */
            &button_pad
                .buttons()
                .into_iter()
                .enumerate()
                .map(|(idx, v)| {
                    if v {
                        // A powerfull algorithm to have fun colors on keypress
                        let mut color = 0x00;
                        if idx % 2 == 0 {
                            color += 0xff;
                        }
                        if idx % 3 == 0 {
                            color += 0xff << 8;
                        }
                        if idx % 5 == 0 {
                            color += 0xff << 16;
                        }
                        if color == 0x00 {
                            color = 0xffffff;
                        }
                        color
                    } else {
                        0x000000
                    }
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )?;
    }
}
