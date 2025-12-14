#![no_std]
#![no_main]

use panic_halt as _;

mod tm1638;
mod ultrasonic;
use tm1638::{SimpleTM1638, display_distance};
use ultrasonic::measure_distance;

#[arduino_hal::entry]
fn main() -> ! {
    let dp: arduino_hal::Peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins: arduino_hal::Pins = arduino_hal::pins!(dp);

    // Built-in LED on pin 13
    let mut built_in_led = pins.d13.into_output();
    
    // External LED on pin 12 (connect LED + resistor between pin 12 and GND)
    let mut external_led = pins.d12.into_output();
    
    // Ultrasonic sensor pins
    // TRIG pin: D11 (send trigger pulse)
    // ECHO pin: D10 (receive echo pulse)
    let mut trig = pins.d11.into_output();
    let mut echo = pins.d10.into_floating_input();
    
    // TM1638 7-segment display pins
    // CLK pin: D9
    // DIO pin: D8
    // STB pin: D7 (strobe/chip select)
    let mut clk = pins.d9.into_output();
    let mut dio = pins.d8.into_output();
    let mut stb = pins.d7.into_output();
    
    // Create display controller
    let mut display = SimpleTM1638 {
        clk: &mut clk,
        dio: &mut dio,
        stb: &mut stb,
    };

    loop {
        // Measure distance from ultrasonic sensor
        let distance_cm = measure_distance(&mut trig, &mut echo);
        
        // Display distance on TM1637 display
        display_distance(&mut display, distance_cm);
        
        // Calculate blink delay based on distance
        // Closer object = faster blinking (shorter delay)
        // Farther object = slower blinking (longer delay)
        // Range: 2cm to 400cm typical
        let blink_delay = calculate_blink_delay(distance_cm);
        
        // Toggle both LEDs with dynamic delay
        let _ = built_in_led.toggle();
        let _ = external_led.toggle();
        arduino_hal::delay_ms(blink_delay.into());
    }
}

// Calculate blink delay based on distance
// Closer object = shorter delay (faster blinking)
// Farther object = longer delay (slower blinking)
fn calculate_blink_delay(distance_cm: u16) -> u16 {
    // Map distance to blink delay
    // At 5cm: 100ms (very fast)
    // At 20cm: 300ms (fast)
    // At 50cm: 500ms (normal)
    // At 100cm+: 1000ms (slow)
    
    match distance_cm {
        0..=2 => 50,        // Very close: very fast
        3..=4 => 100,       // Close: fast
        5..=8 => 200,      // Medium-close: medium-fast
        9..=16 => 400,      // Medium: normal
        17..=32 => 700,     // Far: slow
        _ => 1000,           // Very far: very slow
    }
}
