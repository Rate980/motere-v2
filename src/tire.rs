use std::time::Duration;

use rppal::gpio::OutputPin;
use tokio::time::sleep;

pub struct Tire {
    pin_1: OutputPin,
    pin_2: OutputPin,
    is_stop: bool,
}

impl Tire {
    pub fn new(mut pin_1: OutputPin, mut pin_2: OutputPin) -> Self {
        pin_1.set_low();
        pin_2.set_low();
        Self {
            pin_1,
            pin_2,
            is_stop: true,
        }
    }
    async fn check(&mut self) {
        if !self.is_stop {
            self.stop().await;
        }
    }

    pub async fn backward(&mut self) {
        self.check().await;
        self.pin_1.set_high();
        self.pin_2.set_low();
        self.is_stop = false;
    }

    pub async fn forward(&mut self) {
        self.check().await;
        self.pin_1.set_low();
        self.pin_2.set_high();
        self.is_stop = false;
    }

    pub async fn stop(&mut self) {
        self.pin_1.set_low();
        self.pin_2.set_low();
        self.is_stop = true;
        sleep(Duration::from_micros(10)).await;
    }
    pub async fn blake(&mut self) {
        self.check().await;
        self.pin_1.set_high();
        self.pin_2.set_high();
        self.is_stop = false;
    }
}
