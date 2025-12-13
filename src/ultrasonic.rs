use embedded_hal::digital::{InputPin, OutputPin};

// Measure distance using ultrasonic sensor
// Returns distance in centimeters
pub fn measure_distance<T: OutputPin, U: InputPin>(
    trig: &mut T,
    echo: &mut U,
) -> u16 {
    // Send 10 microsecond pulse on TRIG pin
    let _ = trig.set_high();
    arduino_hal::delay_us(10);
    let _ = trig.set_low();
    
    // Wait for echo to go high
    let mut timeout = 0u16;
    while echo.is_low().unwrap_or(false) && timeout < 1000 {
        timeout += 1;
        arduino_hal::delay_us(1);
    }
    
    // Measure pulse duration
    let mut pulse_duration = 0u16;
    timeout = 0;
    while echo.is_high().unwrap_or(false) && timeout < 30000 {
        pulse_duration += 1;
        timeout += 1;
        arduino_hal::delay_us(1);
    }
    
    // Calculate distance in cm
    // Speed of sound = 343 m/s = 0.0343 cm/µs
    // Distance = (pulse_duration / 2) * 0.0343 ≈ pulse_duration / 58
    // Divide by 2 because sound travels to object and back
    let distance_cm = pulse_duration / 58;
    
    distance_cm
}
