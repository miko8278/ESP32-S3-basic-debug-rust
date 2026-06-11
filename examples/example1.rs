#![no_std]
#![no_main]


use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_println as _; 

esp_bootloader_esp_idf::esp_app_desc!();

// Die asynchrone Task, die alle 2 Sekunden feuert
#[embassy_executor::task]
async fn heartbeat_task() {
    loop {
        //info!("Hello CYD via defmt alle 2 Sekunden!");
        println!("Hello from ESP32!");
        Timer::after(Duration::from_secs(2)).await;
    }
}

// Einstieg über das offizielle esp-rtos Makro
#[esp_rtos::main]
async fn main(spawner: Spawner) {

    // Initialisiert Clocks und Peripherals (v1.1 Standard)
    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    // Software-Interrupts und Timer für den asynchronen Executor holen
    let sw_int = esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);

    // Initialisiert den Embassy-Zeittreiber auf Hardwareebene
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    // Starte unsere asynchrone Schleife
    spawner.spawn(heartbeat_task().unwrap());
}
