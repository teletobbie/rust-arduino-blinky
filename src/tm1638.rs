use embedded_hal::digital::OutputPin;

// Simple TM1638 display controller struct
pub struct SimpleTM1638<'a, T: OutputPin, U: OutputPin, V: OutputPin> {
    pub clk: &'a mut T,
    pub dio: &'a mut U,
    pub stb: &'a mut V,
}

impl<'a, T: OutputPin, U: OutputPin, V: OutputPin> SimpleTM1638<'a, T, U, V> {
    // 7-segment digit patterns (common cathode)
    const DIGITS: [u8; 10] = [
        0x3F, // 0: 0011_1111
        0x06, // 1: 0000_0110
        0x5B, // 2: 0101_1011
        0x4F, // 3: 0100_1111
        0x66, // 4: 0110_0110
        0x6D, // 5: 0110_1101
        0x7D, // 6: 0111_1101
        0x07, // 7: 0000_0111
        0x7F, // 8: 0111_1111
        0x6F, // 9: 0110_1111
    ];

    pub fn write_digits(&mut self, digits: &[u8; 4]) {
        // STB low to select chip
        let _ = self.stb.set_low();
        arduino_hal::delay_us(10);
        
        // Send data command (0x40 = auto increment)
        self.send_byte(0x40);
        
        // STB high then low again for address
        let _ = self.stb.set_high();
        arduino_hal::delay_us(10);
        let _ = self.stb.set_low();
        arduino_hal::delay_us(10);
        
        // Send start address (0xC0 = address 0)
        self.send_byte(0xC0);
        
        // Send 4 digits with spacing for TM1638 (each digit is at position 0,2,4,6)
        for (idx, &digit) in digits.iter().enumerate() {
            let pattern = if digit == 0 { 0x00 } else { Self::DIGITS[digit as usize] };
            self.send_byte(pattern);
            
            // Send dummy byte for the next position (TM1638 has 8 segments per position)
            if idx < 3 {
                self.send_byte(0x00);
            }
        }
        
        // STB high to latch data
        let _ = self.stb.set_high();
        arduino_hal::delay_us(10);
        
        // Enable display
        let _ = self.stb.set_low();
        arduino_hal::delay_us(10);
        self.send_byte(0x8F); // Display on, max brightness
        let _ = self.stb.set_high();
        arduino_hal::delay_us(10);
    }

    fn send_byte(&mut self, byte: u8) {
        for i in 0..8 {
            // Set data line for this bit (LSB first)
            if (byte >> i) & 1 == 1 {
                let _ = self.dio.set_high();
            } else {
                let _ = self.dio.set_low();
            }
            
            arduino_hal::delay_us(2);
            
            // Clock pulse
            let _ = self.clk.set_high();
            arduino_hal::delay_us(5);
            let _ = self.clk.set_low();
            arduino_hal::delay_us(5);
        }
    }
}

// Display distance on TM1638 7-segment display
pub fn display_distance<T: OutputPin, U: OutputPin, V: OutputPin>(display: &mut SimpleTM1638<T, U, V>, distance_cm: u16) {
    // Limit distance to 9999 for 4-digit display
    let distance = if distance_cm > 9999 { 9999 } else { distance_cm };
    
    // Convert distance to 4-digit array for display
    let thousands = (distance / 1000) as u8;
    let hundreds = ((distance / 100) % 10) as u8;
    let tens = ((distance / 10) % 10) as u8;
    let ones = (distance % 10) as u8;
    
    // Create array of digits to display
    let mut digits = [0u8; 4];
    
    // Only show leading digits if they're non-zero
    if thousands > 0 {
        digits[0] = thousands;
        digits[1] = hundreds;
        digits[2] = tens;
        digits[3] = ones;
    } else if hundreds > 0 {
        digits[0] = hundreds;
        digits[1] = tens;
        digits[2] = ones;
        digits[3] = 0; // blank
    } else if tens > 0 {
        digits[0] = tens;
        digits[1] = ones;
        digits[2] = 0; // blank
        digits[3] = 0; // blank
    } else {
        digits[0] = 0; // blank
        digits[1] = 0; // blank
        digits[2] = 0; // blank
        digits[3] = ones;
    }
    
    // Update display
    display.write_digits(&digits);
}
