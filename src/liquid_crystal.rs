// commands
const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_RETURNHOME: u8 = 0x02;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_CURSORSHIFT: u8 = 0x10;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_SETCGRAMADDR: u8 = 0x40;
const LCD_SETDDRAMADDR: u8 = 0x80;

// flags for display entry mode
const LCD_ENTRYRIGHT: u8 = 0x00;
const LCD_ENTRYLEFT: u8 = 0x02;
const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;

// flags for display on/off control
const LCD_DISPLAYON: u8 = 0x04;
const LCD_DISPLAYOFF: u8 = 0x00;
const LCD_CURSORON: u8 = 0x02;
const LCD_CURSOROFF: u8 = 0x00;
const LCD_BLINKON: u8 = 0x01;
const LCD_BLINKOFF: u8 = 0x00;

// flags for display/cursor shift
const LCD_DISPLAYMOVE: u8 = 0x08;
const LCD_CURSORMOVE: u8 = 0x00;
const LCD_MOVERIGHT: u8 = 0x04;
const LCD_MOVELEFT: u8 = 0x00;

// flags for function set
const LCD_8BITMODE: u8 = 0x10;
const LCD_4BITMODE: u8 = 0x00;
const LCD_2LINE: u8 = 0x08;
const LCD_1LINE: u8 = 0x00;
const LCD_5x10DOTS: u8 = 0x04;
const LCD_5x8DOTS: u8 = 0x00;

const OUTPUT: u8 = 0x1;
const INPUT: u8 = 0x0;
const INPUT_PULLUP: u8 = 0x2;

const HIGH: u8 = 0x1;
const LOW : u8 = 0x0;


// When the display powers up, it is configured as follows:
//
// 1. Display clear
// 2. Function set:
//    DL = 1; 8-bit interface data
//    N = 0; 1-line display
//    F = 0; 5x8 dot character font
// 3. Display on/off control:
//    D = 0; Display off
//    C = 0; Cursor off
//    B = 0; Blinking off
// 4. Entry mode set:
//    I/D = 1; Increment by 1
//    S = 0; No shift
//
// Note, however, that resetting the Arduino doesn't reset the LCD, so we
// can't assume that its in that state when a sketch starts (and the
// LiquidCrystal constructor is called).
pub struct LiquidCrystal {
    rs_pin: u8, // LOW: command.  HIGH: character.
    rw_pin: u8, // LOW: write to LCD.  HIGH: read from LCD.
    enable_pin: u8, // activated by a HIGH pulse.
    data_pins: [u8; 8],

    displayfunction: u8,
    displaycontrol: u8,
    displaymode: u8,

    initialized: u8,

    numlines: u8,
    row_offsets: [u8; 4],
}

impl Default for LiquidCrystal {
    fn default() -> Self {
        return LiquidCrystal {
            rs_pin: 0,
            rw_pin: 0,
            enable_pin: 0,
            data_pins: [0; 8],
            displayfunction: 0,
            displaycontrol: 0,
            displaymode: 0,
            initialized: 0,
            numlines: 0,
            row_offsets: [0; 4],
        }
    }
}

impl LiquidCrystal {
    pub fn liquid_crystal_4(rs: u8, enable: u8,
                            d0: u8, d1: u8, d2: u8, d3: u8,
                            d4: u8, d5: u8, d6: u8, d7: u8) -> LiquidCrystal {
        let mut lc = LiquidCrystal::default();
        lc.init(0, rs, 255, enable, d0, d1, d2, d3, d4, d5, d6, d7);
        return lc;
    }


    pub fn liquid_crystal_3(rs: u8, rw: u8, enable: u8,
                            d0: u8, d1: u8, d2: u8, d3: u8,
                            d4: u8, d5: u8, d6: u8, d7: u8) -> LiquidCrystal {

        let mut lc = LiquidCrystal::default();
        lc.init(0, rs, rw, enable, d0, d1, d2, d3, d4, d5, d6, d7);
        return lc;
    }
    pub fn liquid_crystal_2(rs: u8, rw: u8, enable: u8,
                            d0: u8, d1: u8, d2: u8, d3: u8) -> LiquidCrystal {
        let mut lc = LiquidCrystal::default();
        lc.init(1, rs, rw, enable, d0, d1, d2, d3, 0, 0, 0, 0);

        return lc;
    }
    pub fn liquid_crystal_1(rs: u8, enable: u8,
                            d0: u8, d1: u8, d2: u8, d3: u8) -> LiquidCrystal {
        let mut lc = LiquidCrystal::default();
        lc.init(1, rs, 255, enable, d0, d1, d2, d3, 0, 0, 0, 0);

        return lc;
    }

    pub fn init(&mut self, fourbitmode: u8, rs: u8, rw: u8, enable: u8,
                d0: u8, d1: u8, d2: u8, d3: u8,
                d4: u8, d5: u8, d6: u8, d7: u8) {
        self.rs_pin = rs;
        self.rw_pin = rw;
        self.enable_pin = enable;

        self.data_pins[0] = d0;
        self.data_pins[1] = d1;
        self.data_pins[2] = d2;
        self.data_pins[3] = d3;
        self.data_pins[4] = d4;
        self.data_pins[5] = d5;
        self.data_pins[6] = d6;
        self.data_pins[7] = d7;

        if fourbitmode > 0 {
            self.displayfunction = LCD_4BITMODE | LCD_1LINE | LCD_5x8DOTS;
        } else {
            self.displayfunction = LCD_8BITMODE | LCD_1LINE | LCD_5x8DOTS;
        }

        self.begin(16, 1, None);
    }

    pub fn begin(&mut self, cols: u8, lines: u8, dotsize: Option<u8>) {

        let dotsize = dotsize.unwrap_or(LCD_5x8DOTS);
        if lines > 1 {
            self.displayfunction |= LCD_2LINE;
        }

        self.numlines = lines;

        self.setRowOffsets(0x00, 0x40, 0x00 + cols, 0x40 + cols);

        // for some 1 line displays you can select a 10 pixel high font
        if dotsize != LCD_5x8DOTS && lines == 1 {
            self.displayfunction |= LCD_5x10DOTS;
        }

        pinMode(self.rs_pin, OUTPUT);
        // we can save 1 pin by not using RW. Indicate by passing 255 instead of pin#
        if (self.rw_pin != 255) {
            pinMode(self.rw_pin, OUTPUT);
        }
        pinMode(self.enable_pin, OUTPUT);

        // Do these once, instead of every time a character is drawn for speed reasons.
        let end = if self.displayfunction & LCD_8BITMODE > 0 { 8 } else { 4 };
        for i in 0..end {
            pinMode(self.data_pins[i], OUTPUT);
        }

        // SEE PAGE 45/46 FOR INITIALIZATION SPECIFICATION!
        // according to datasheet, we need at least 40ms after power rises above 2.7V
        // before sending commands. Arduino can turn on way before 4.5V so we'll wait 50
        self.delayMicroseconds(50000);

        // Now we pull both RS and R/W low to begin commands
        self.digitalWrite(self.rs_pin, LOW);
        self.digitalWrite(self.enable_pin, LOW);
        if self.rw_pin != 255 {
            self.digitalWrite(self.rw_pin, LOW);
        }

        //put the LCD into 4 bit or 8 bit mode
        if self.displayfunction & LCD_8BITMODE == 0 {
            // this is according to the hitachi HD44780 datasheet
            // figure 24, pg 46

            // we start in 8bit mode, try to set 4 bit mode
            self.write4bits(0x03);
            self.delayMicroseconds(4500); // wait min 4.1ms

            // second try
            self.write4bits(0x03);
            self.delayMicroseconds(4500); // wait min 4.1ms

            // third go!
            self.write4bits(0x03);
            self.delayMicroseconds(150);

            // finally, set to 4-bit interface
            self.write4bits(0x02);
        } else {
            // this is according to the hitachi HD44780 datasheet
            // page 45 figure 23

            // Send function set command sequence
            self.command(LCD_FUNCTIONSET | self.displayfunction);
            self.delayMicroseconds(4500);  // wait more than 4.1ms

            // second try
            self.command(LCD_FUNCTIONSET | self.displayfunction);
            self.delayMicroseconds(150);

            // third go
            self.command(LCD_FUNCTIONSET | self.displayfunction);
        }

        // finally, set # lines, font size, etc.
        command(LCD_FUNCTIONSET | self.displayfunction);

        // turn the display on with no cursor or blinking default
        self.displaycontrol = LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF;
        display();

        // clear it off
        clear();

        // Initialize to default text direction (for romance languages)
        self.displaymode = LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT;
        // set the entry mode
        command(LCDself.ENTRYMODESET | _displaymode);


    }

    pub fn clear() {
    }
    pub fn home() {
    }

    pub fn noDisplay() {
    }
    pub fn display() {
    }
    pub fn noBlink() {
    }
    pub fn blink() {
    }
    pub fn noCursor() {
    }
    pub fn cursor() {
    }
    pub fn scrollDisplayLeft() {
    }
    pub fn scrollDisplayRight() {
    }
    pub fn leftToRight() {
    }
    pub fn rightToLeft() {
    }
    pub fn autoscroll() {
    }
    pub fn noAutoscroll() {
    }

    pub fn setRowOffsets(row1: isize, row2: isize, row3: isize, row4: isize) {
    }

    pub fn createChar(location: u8, charmap: &[u8], len: u8) {
    }

    pub fn setCursor(col: u8, row: u8) {
    }
    pub fn write(value: u8) -> usize {
        return 0;
    }

    pub fn command(value: u8) {
    }

    fn send(value: u8,mode: u8) {
    }
    fn write4bits(value: u8) {
    }
    fn write8bits(value: u8) {
    }
    fn pulseEnable() {
    }
}

