#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _; // Enable RTT transport for defmt
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

#[embassy_executor::task]
async fn heartbeat_task() {
    loop {
        info!("Hello CYD via defmt alle 2 Sekunden!");
        Timer::after(Duration::from_secs(2)).await;
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // Initialize the HAL
    let _peripherals = esp_hal::init(esp_hal::Config::default());

    info!("Bare-Metal Runtime aktiv. Starte Task...");

    spawner.spawn(heartbeat_task());
}