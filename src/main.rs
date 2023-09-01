pub mod codec;
pub mod settings;
mod tire;

use codec::EnumCodec;
use futures::StreamExt;
use rppal::gpio::Gpio;
#[allow(unused_imports)]
use settings::{BLE_PATH, BLE_RATE, INFRARED_PATH, INFRARED_RATE, SPRESENSE_PATH, SPRESENSE_RATE};
use tire::Tire;

use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::codec::{Decoder, Framed};

#[inline]
fn tty_setting(path: &str, rate: u32) -> tokio_serial::Result<Framed<SerialStream, EnumCodec>> {
    let mut port = tokio_serial::new(path, rate).open_native_async()?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    Ok(EnumCodec.framed(port))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut infrared = tty_setting(INFRARED_PATH, INFRARED_RATE)?;
    // let mut ble = tty_setting(BLE_PATH, BLE_RATE)?;
    // let mut spresense = tty_setting(SPRESENSE_PATH, SPRESENSE_RATE)?;

    let gpio = Gpio::new()?;
    let mut right_tire = Tire::new(
        gpio.get(settings::R1)?.into_output(),
        gpio.get(settings::R2)?.into_output(),
    );

    let mut left_tire = Tire::new(
        gpio.get(settings::L1)?.into_output(),
        gpio.get(settings::L2)?.into_output(),
    );

    // right_tire.forward().await;
    // left_tire.forward().await;
    // right_tire.stop().await;
    // left_tire.stop().await;
    // loop {
    //     sleep(Duration::from_millis(1000)).await;
    // }

    // motor task
    let handle = tokio::spawn(async move {
        loop {
            let line_result = match infrared.next().await {
                Some(x) => x,
                None => {
                    continue;
                }
            };
            // let line = line_result.expect("Failed to read line");
            let mut line = match line_result {
                Ok(x) => x,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };
            line &= 0b11111;
            match line {
                0b00100 | 0b01110 | 0b01100 | 0b00110 => {
                    println!("forward");
                    right_tire.forward().await;
                    left_tire.forward().await;
                }
                0b00001 | 0b00011 | 0b00111 => {
                    println!("right");
                    right_tire.forward().await;
                    left_tire.backward().await;
                }
                0b10000 | 0b11000 | 0b11100 => {
                    println!("left");
                    right_tire.backward().await;
                    left_tire.forward().await;
                }
                0b00000 => {
                    println!("search");
                    right_tire.backward().await;
                    left_tire.forward().await;
                }
                _ => (),
            }
        }
    });

    // BLE task
    // tokio::spawn(async move {
    //     while let Some(line_result) = ble.next().await {
    //         let line = line_result.expect("Failed to read line");
    //     }
    // });
    handle.await.unwrap();
    Ok(())
}
