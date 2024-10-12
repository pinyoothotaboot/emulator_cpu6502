/*
    CPU 6502 process status flags : http://wiki.nesdev.com/w/index.php/Status_flags
     processor status (PS) <-- Byte
     7 6 5 4 3 2 1 0 <-- Bit
     N V _ B D I Z C <-- Flag
     | |   | | | | +--- Carry Flag                     0 = FALSE                 1 = TRUE
     | |   | | | +----- Zero Flag                      0 = RESULT NOT ZERO       1 = RESULT ZERO
     | |   | | +------- Interrupt Disable (IRQ)        0 = ENABLE                1 = DISABLE
     | |   | +--------- Decimal Mode (not used on NES) 0 = FALSE                 1 = TRUE
     | |   +----------- Break Command                  0 = NO BREAK              1 = BREAK
     | +--------------- Overflow Flag                  0 = FALSE                 1 = TRUE
     +----------------- Negative Flag                  0 = POSITIVE              1 = NEGATIVE

*/

pub struct ProcessorStatus {
    N: u8,
    V: u8,
    U: u8,
    B: u8,
    D: u8,
    I: u8,
    Z: u8,
    C: u8,
}

impl ProcessorStatus {
    pub fn new() -> Self {
        ProcessorStatus {
            N: 0x01,
            V: 0x01,
            U: 0x01,
            B: 0x01,
            D: 0x01,
            I: 0x01,
            Z: 0x01,
            C: 0x01,
        }
    }

    pub fn reset(&mut self) {
        self.N = 0x00;
        self.V = 0x00;
        self.U = 0x00;
        self.B = 0x00;
        self.D = 0x00;
        self.I = 0x00;
        self.Z = 0x00;
        self.C = 0x00;
    }

    pub fn get_status(&self) -> u8 {
        return self.N << 7
            | self.V << 6
            | self.U << 5
            | self.B << 4
            | self.D << 3
            | self.I << 2
            | self.Z << 1
            | self.C;
    }

    pub fn set_overflow(&mut self) {
        self.V = 0x01;
    }

    pub fn unset_overflow(&mut self) {
        self.V = 0x00;
    }

    pub fn set_unused(&mut self) {
        self.U = 0x01;
    }

    pub fn unset_unused(&mut self) {
        self.U = 0x00;
    }

    pub fn set_break_command(&mut self) {
        self.B = 0x01;
    }

    pub fn unset_break_command(&mut self) {
        self.B = 0x00;
    }

    pub fn set_decimal_mode(&mut self) {
        self.D = 0x01;
    }

    pub fn unset_decimal_mode(&mut self) {
        self.D = 0x00;
    }

    pub fn set_interrupt_disable(&mut self) {
        self.I = 0x01;
    }

    pub fn unset_interrupt_disable(&mut self) {
        self.I = 0x00;
    }

    pub fn set_zero(&mut self) {
        self.Z = 0x01;
    }

    pub fn unset_zero(&mut self) {
        self.Z = 0x00;
    }

    pub fn set_carry(&mut self) {
        self.C = 0x01;
    }

    pub fn unset_carry(&mut self) {
        self.C = 0x00;
    }
}
