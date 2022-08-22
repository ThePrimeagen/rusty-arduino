#![no_std]
#![no_main]

use ag_lcd::{LcdDisplay, Display, Blink, Cursor};
use panic_halt as _;
use arduino_hal::prelude::*;

fn read_str<T:embedded_hal::serial::Read<u8>> (reader: &mut T, buf: &mut [u8], offset: usize) -> (usize, bool) {
    let mut curr_byte = reader.read();
    let mut len = 0;
    if curr_byte.is_err() {
        return (offset, false);
    }

    let mut done = false;
    while let Ok(byte) = curr_byte {
        if byte == 0 {
            done = true;
            break;
        }

        buf[offset + len] = byte;
        len += 1;

        curr_byte = reader.read();
    }

    return (offset + len, done);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let rs = pins.d12.into_output().downgrade();
    //let rw = pins.d11.into_output().downgrade();
    let en = pins.d11.into_output().downgrade();
    let d4 = pins.d5.into_output().downgrade();
    let d5 = pins.d4.into_output().downgrade();
    let d6 = pins.d3.into_output().downgrade();
    let d7 = pins.d2.into_output().downgrade();
    // let mut led = pins.d13.into_output();

    let delay = arduino_hal::Delay::new();

    let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
        // .with_full_bus(d0, d1, d2, d3, d4, d5, d6, d7)
        .with_half_bus(d4, d5, d6, d7)
        .with_display(Display::On)
        .with_blink(Blink::On)
        .with_cursor(Cursor::On)
        // .with_rw(d10) // optional (set to GND if not provided)
        .build();

    lcd.set_cursor(Cursor::Off);
    lcd.set_blink(Blink::Off);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    match ufmt::uwriteln!(&mut serial, "{} bytes available", 69420) {
        Ok(_) => {
            lcd.print("JS sucks");
        },
        Err(_) => {
            // WHAT?
            lcd.print("JS is Good");
        },
    }
    /*

    let mut count = 0;
    loop {
        arduino_hal::delay_ms(1000);
        led.toggle();

        if count == 0 {
            let _byte = serial.read_byte();
        }

        count += 1;
    }
    */

    let mut led = pins.d13.into_output();
    let mut buf = [0; 128];
    let mut offset = 0;
        lcd.set_position(0, 0);

    loop {
        let (o, _done) = read_str(&mut serial, &mut buf, offset);

        offset += o;

        /*
        // lcd.print("test loop");
        if done {
            for idx in 0..offset {
                serial.write_byte(buf[idx]);
            }
        }

        if n > 0 {
            if let Ok(str) = core::str::from_utf8(&buf[..n]) {
                lcd.print(str);
            }
        }
        */

        led.toggle();
        arduino_hal::delay_ms(1000);
    }
 }
