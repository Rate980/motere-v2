use rppal::gpio::{self, Gpio};

fn main() -> gpio::Result<()> {
    let gpio = Gpio::new()?;
    gpio.get(2)?.into_output_low();
    gpio.get(3)?.into_output_low();

    gpio.get(23)?.into_output_low();
    gpio.get(24)?.into_output_low();
    Ok(())
}
