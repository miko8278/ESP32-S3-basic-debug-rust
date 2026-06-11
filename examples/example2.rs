#![no_std]
#![no_main]

use esp_hal::lcd_cam::{
    LcdCam,
    lcd::{
        dpi::*,
        ClockMode,
        Phase,
        Polarity,
    },
};

use defmt::*;
//use defmt_rtt as _; // <-- DIESE ZEILE HINZUFÜGEN!
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_println as _; // <-- WICHTIG: Das sorgt dafür, dass der Auto-Logger gelinkt wird!
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

    let config = dpi::Config::default()
    .with_frequency(Rate::from_mhz(10))
    .with_clock_mode(ClockMode {
        polarity: Polarity::IdleLow,
        phase: Phase::ShiftLow,
    })
    .with_format(Format {
        enable_2byte_mode: true,
        ..Default::default()
    })
    .with_timing(FrameTiming {
        horizontal_active_width: 800,
        horizontal_total_width: 1056,

        horizontal_blank_front_porch: 256,

        vertical_active_height: 480,
        vertical_total_height: 525,

        vertical_blank_front_porch: 45,

        hsync_width: 20,
        vsync_width: 10,

        hsync_position: 0,
    });
    // Initialisiert Clocks und Peripherals (v1.1 Standard)
    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    // Software-Interrupts und Timer für den asynchronen Executor holen
    let sw_int = esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);

    // Initialisiert den Embassy-Zeittreiber auf Hardwareebene
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    //info!("Bare-Metal Runtime aktiv. Starte Task...");

    // Starte unsere asynchrone Schleife
    spawner.spawn(heartbeat_task().unwrap());
}
